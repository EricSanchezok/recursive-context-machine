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
步骤 2：抓取链接原文
优先使用 requests+BeautifulSoup；不行则用 LangChain WebBaseLoader
输出—— raw_texts/<随机ID>.txt: 抓取的原文内容
"""
import logging
import sys
from pathlib import Path
sys.path.append(str(Path(__file__).parent.parent))

import requests
from bs4 import BeautifulSoup

from langchain_community.document_loaders import WebBaseLoader
from tqdm import tqdm

from utils import retry_async

logging.basicConfig(level=logging.ERROR)
logger = logging.getLogger(__name__)


@retry_async(attempts=2)
def _load_with_requests_bs4(url: str) -> str | None:
    """用 requests + BeautifulSoup 抓取页面文本（零 API key）"""
    try:
        headers = {
            "User-Agent": (
                "Mozilla/5.0 (Windows NT 10.0; Win64; x64) "
                "AppleWebKit/537.36 (KHTML, like Gecko) "
                "Chrome/120.0.0.0 Safari/537.36"
            )
        }
        resp = requests.get(url, headers=headers, timeout=30)
        resp.raise_for_status()
        soup = BeautifulSoup(resp.text, "html.parser")
        # 移除干扰元素
        for tag in soup(["script", "style", "nav", "footer", "header", "aside"]):
            tag.decompose()
        text = soup.get_text(separator="\n", strip=True)
        return text if text.strip() else None
    except Exception as e:
        logger.warning(f"requests+bs4 抓取失败 {url}: {e}")
        return None


@retry_async(attempts=2)
def _load_with_langchain(url: str) -> str | None:
    loader = WebBaseLoader(url)
    docs = loader.load()
    return docs[0].page_content if docs else None


def scrape_all(df_citations, out_dir: str | Path = "raw_texts"):
    """抓取所有链接内容，基于随机ID避免重复抓取"""
    Path(out_dir).mkdir(exist_ok=True)

    for _, row in tqdm(df_citations.iterrows(), total=len(df_citations), desc="Scraping"):
        random_id = row.ID
        url = row.url
        out_path = Path(out_dir) / f"{random_id}.txt"
        
        # 如果文件已存在，跳过抓取
        if out_path.exists():
            logger.info(f"[✓] ID {random_id} 已缓存，跳过抓取")
            continue

        logger.info(f"[→] 抓取 ID {random_id}: {url}")
        
        # 抓取内容：先用 requests+bs4，失败则用 LangChain
        text = None
        try:
            text = _load_with_requests_bs4(url)
            if not text:
                text = _load_with_langchain(url)
        except Exception as e:
            logger.error(f"[!] 抓取失败: {url}，错误: {e}")

        if text:
            out_path.write_text(text, encoding="utf-8")
            logger.info(f"[✓] 成功抓取: {random_id}.txt")
        else:
            logger.warning(f"[!] 抓取失败: {url}")
