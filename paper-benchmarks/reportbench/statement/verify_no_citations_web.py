# Copyright (c) 2025 Bytedance Ltd. and/or its affiliates
#
# Licensed under the Apache License, Version 2.0 (the "License");
# you may not use this file except in compliance with the License.
# You may obtain a copy of the License at
#
#     http://www.apache.org/licenses/LICENSE-2.0
#
# Unless required by applicable law or agreed to in writing, software
# distributed under the License is distributed on an "AS IS" BASIS,
# WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
# See the License for the specific language governing permissions and
# limitations under the License.

"""
步骤 3b：使用联网LLM验证无引用表述
使用两个联网LLM，每个生成3次，共6个判断进行投票
输出—— no_citations_web_verification.csv: 详细验证结果
输出—— no_citations_web_final.csv: 最终统计结果
"""
import json
import logging
import sys
from pathlib import Path
sys.path.append(str(Path(__file__).parent.parent))

import pandas as pd
from tqdm import tqdm
from collections import Counter
from concurrent.futures import ThreadPoolExecutor, as_completed
import time

from statement.prompts import PROMPT_VERIFY_STATEMENT_SIMPLE, PROMPT_VERIFY_STATEMENT_WEB
from utils import build_web_llms, save_csv, post_process_json

logging.basicConfig(level=logging.INFO)
logger = logging.getLogger(__name__)


def verify_statement_with_web_llm(statement: str, web_llm_client, attempt: int = 1, prompt_template: str = PROMPT_VERIFY_STATEMENT_SIMPLE) -> tuple[bool, str]:
    """使用LLM验证单个表述（根据 prompt_template 决定是否联网）"""
    messages = [
        {
            "role": "user", 
            "content": prompt_template.format(statement=statement)
        }
    ]
    
    try:
        response = web_llm_client.generate(messages)
        
        # 处理JSON响应 - 增强错误处理
        post_processed = post_process_json(response)
        
        # 尝试多种方式处理JSON
        try:
            # 首先尝试直接解析
            data = json.loads(post_processed)
        except json.JSONDecodeError as e:
            logger.warning(f"直接JSON解析失败 (尝试 {attempt}): {e}")
            
            # 尝试清理响应内容
            cleaned_response = clean_json_response(post_processed)
            try:
                data = json.loads(cleaned_response)
                logger.info(f"清理后JSON解析成功 (尝试 {attempt})")
            except json.JSONDecodeError as e2:
                logger.error(f"清理后JSON解析仍失败 (尝试 {attempt}): {e2}")
                logger.error(f"原始响应: {response[:200]}...")
                logger.error(f"后处理响应: {post_processed[:200]}...")
                logger.error(f"清理响应: {cleaned_response[:200]}...")
                
                # 作为最后的尝试，使用正则表达式提取JSON
                try:
                    extracted_data = extract_json_with_regex(post_processed)
                    if extracted_data:
                        data = extracted_data
                        logger.info(f"正则表达式提取JSON成功 (尝试 {attempt})")
                    else:
                        raise ValueError("无法从响应中提取有效JSON")
                except Exception as e3:
                    logger.error(f"正则表达式提取也失败 (尝试 {attempt}): {e3}")
                    return False, f"JSON解析失败: {str(e)}"
        
        decision = data.get("decision", False)
        reason = data.get("reason", "无法判断")
        
        # 确保decision是布尔类型
        if isinstance(decision, str):
            decision = decision.lower() in ['true', '正确', 'yes', '1']
        
        return decision, reason
        
    except Exception as e:
        logger.error(f"验证失败 (尝试 {attempt}): {e}")
        return False, f"验证过程出错: {str(e)}"


def clean_json_response(response: str) -> str:
    """清理JSON响应内容，自动补全截断的JSON"""
    response = response.strip()

    # 尝试找到JSON对象的开始和结束
    start_idx = response.find('{')
    if start_idx == -1:
        return response

    # 找到第一个完整JSON对象的结束位置
    brace_count = 0
    end_idx = -1
    for i in range(start_idx, len(response)):
        if response[i] == '{':
            brace_count += 1
        elif response[i] == '}':
            brace_count -= 1
            if brace_count == 0:
                end_idx = i + 1
                break

    if end_idx > start_idx:
        return response[start_idx:end_idx]

    # 未闭合 — 尝试补全
    trimmed = response[start_idx:]
    # 确保 reason 字段被正确关闭
    if '"reason"' in trimmed and not trimmed.rstrip().endswith('}'):
        # 补充缺漏的引号和大括号
        repaired = trimmed.rstrip()
        if not repaired.endswith('"'):
            repaired += '"'
        # 尝试找到 decision 字段
        if '"decision"' not in repaired:
            repaired += ', "decision": false'
        repaired += '}'
        return repaired

    return response


def extract_json_with_regex(response: str) -> dict:
    """使用正则表达式提取JSON内容（容忍截断）"""
    import re

    # 尝试匹配 decision 字段 (大小写不敏感, 支持true/false/True/False)
    decision_match = re.search(r'"decision"\s*:\s*(true|false|True|False)', response)
    # 尝试匹配 reason 字段 (容忍截断和换行)
    reason_match = re.search(r'"reason"\s*:\s*"([^"]*)"', response, re.DOTALL)

    if decision_match:
        decision = decision_match.group(1).lower() == 'true'
        if reason_match:
            reason = reason_match.group(1).strip()
        else:
            # 尝试提取reason但不要求完整的闭合引号
            reason_partial = re.search(r'"reason"\s*:\s*"([^"]*)', response)
            reason = reason_partial.group(1).strip() if reason_partial else "无法提取原因"
        return {
            "decision": decision,
            "reason": reason
        }

    return None


def verify_single_statement_task(statement_id: str, statement: str, web_llm, llm_name: str, attempt: int, prompt_template: str) -> dict:
    """单个验证任务，用于并发执行"""
    try:
        decision, reason = verify_statement_with_web_llm(statement, web_llm, attempt, prompt_template)
        
        return {
            "ID": statement_id,
            "statement": statement,
            "llm_model": llm_name,
            "attempt": attempt,
            "decision": decision,
            "reason": reason,
            "success": True
        }
        
    except Exception as e:
        logger.error(f"表述 {statement_id} 使用 {llm_name} 尝试 {attempt} 失败: {e}")
        return {
            "ID": statement_id,
            "statement": statement,
            "llm_model": llm_name,
            "attempt": attempt,
            "decision": False,
            "reason": f"验证失败: {str(e)}",
            "success": False
        }


def verify_no_citations_web(
    df_no_citations: pd.DataFrame,
    verification_csv: str | Path = "no_citations_web_verification.csv",
    final_csv: str | Path = "no_citations_web_final.csv",
    max_workers: int = 10,
    mode: str = "simple"
) -> tuple[pd.DataFrame, pd.DataFrame]:
    """使用 LLM 并发验证所有无引用表述
    
    mode:
      "simple" — 基于模型知识（PROMPT_VERIFY_STATEMENT_SIMPLE）
      "full"   — 基于联网搜索（PROMPT_VERIFY_STATEMENT_WEB，需 WEB_LLM_KEY）
    """
    
    prompt_template = (
        PROMPT_VERIFY_STATEMENT_WEB if mode == "full"
        else PROMPT_VERIFY_STATEMENT_SIMPLE
    )
    
    # 初始化两个 LLM 客户端
    web_llms = build_web_llms()
    verification_results = []
    
    # 准备所有验证任务
    tasks = []
    for _, row in df_no_citations.iterrows():
        statement_id = row.ID
        statement = row.statement
        
        # 为每个LLM创建3次验证任务
        for llm_idx, web_llm in enumerate(web_llms):
            llm_name = web_llm.model_name
            for attempt in range(1, 4):  # 1, 2, 3
                tasks.append((statement_id, statement, web_llm, llm_name, attempt, prompt_template))
    
    total_tasks = len(tasks)
    logger.info(f"开始并发验证 {len(df_no_citations)} 个表述，共 {total_tasks} 个验证任务")
    
    # 使用线程池执行并发验证
    start_time = time.time()
    with ThreadPoolExecutor(max_workers=max_workers) as executor:
        # 提交所有任务
        future_to_task = {
            executor.submit(verify_single_statement_task, *task): task 
            for task in tasks
        }
        
        # 收集结果
        with tqdm(total=total_tasks, desc="并发验证中") as pbar:
            for future in as_completed(future_to_task):
                result = future.result()
                verification_results.append(result)
                
                # 更新进度条
                pbar.set_postfix({
                    'Statement': result['ID'][:10] + '...',
                    'Model': result['llm_model'][:10],
                    'Attempt': result['attempt'],
                    'Result': '✓' if result['decision'] else '✗'
                })
                pbar.update(1)
    
    end_time = time.time()
    logger.info(f"并发验证完成，耗时 {end_time - start_time:.2f} 秒")
    
    # 保存详细验证结果
    df_verification = pd.DataFrame(verification_results)
    save_csv(df_verification, verification_csv)
    
    # 计算每个表述的投票结果
    final_results = []
    
    for statement_id in df_verification['ID'].unique():
        statement_data = df_verification[df_verification['ID'] == statement_id]
        
        if statement_data.empty:
            continue
            
        statement = statement_data.iloc[0]['statement']
        
        # 统计投票结果
        decisions = statement_data['decision'].tolist()
        # 确保决策结果为布尔类型，处理可能的字符串值
        boolean_decisions = []
        for decision in decisions:
            if isinstance(decision, bool):
                boolean_decisions.append(decision)
            elif isinstance(decision, str):
                # 字符串转布尔：true/True/正确 -> True，其他 -> False
                boolean_decisions.append(decision.lower() in ['true', '正确', 'yes', '1'])
            else:
                # 其他类型转布尔
                boolean_decisions.append(bool(decision))
        
        true_votes = sum(boolean_decisions)
        false_votes = len(boolean_decisions) - true_votes
        
        # 判断最终结果
        total_valid_votes = len(boolean_decisions)
        if false_votes > true_votes:
            final_decision = "错误"
            confidence = false_votes / total_valid_votes if total_valid_votes > 0 else 0
        elif true_votes > false_votes:
            final_decision = "正确"
            confidence = true_votes / total_valid_votes if total_valid_votes > 0 else 0
        else:
            final_decision = "平局"
            confidence = 0.5
        
        # 收集所有原因
        reasons = statement_data['reason'].tolist()
        combined_reasons = "; ".join(reasons)
        
        final_results.append({
            "ID": statement_id,
            "statement": statement,
            "true_votes": true_votes,
            "false_votes": false_votes,
            "total_votes": total_valid_votes,
            "final_decision": final_decision,
            "confidence": confidence,
            "reasons": combined_reasons
        })
    
    # 保存最终结果
    df_final = pd.DataFrame(final_results)
    save_csv(df_final, final_csv)
    
    # 计算总体统计
    if not df_final.empty:
        correct_count = len(df_final[df_final['final_decision'] == '正确'])
        incorrect_count = len(df_final[df_final['final_decision'] == '错误'])
        tie_count = len(df_final[df_final['final_decision'] == '平局'])
        
        total_statements = len(df_final)
        accuracy_rate = correct_count / total_statements if total_statements > 0 else 0
        
        mode_label = "简单模式(LLM)" if mode == "simple" else "完整模式(联网LLM)"
        print(f"\n==============================")
        print(f"LLM验证结果 ({mode_label}):")
        print(f"总表述数: {total_statements}")
        print(f"投票结果为正确: {correct_count} ({correct_count/total_statements:.1%})")
        print(f"投票结果为错误: {incorrect_count} ({incorrect_count/total_statements:.1%})")
        print(f"投票平局: {tie_count} ({tie_count/total_statements:.1%})")
        print(f"正确率 (ACC): {accuracy_rate:.2%}")
        print(f"==============================\n")
    
    return df_verification, df_final 