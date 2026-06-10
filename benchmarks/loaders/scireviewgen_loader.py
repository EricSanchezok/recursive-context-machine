import os
import json
import pickle
from typing import List, Dict, Optional

class SciReviewGenLoader:
    def __init__(self, data_dir: str = "d:\\RCM\\benchmarks\\datasets\\SciReviewGen"):
        self.data_dir = data_dir
        self.split_survey_df_path = os.path.join(data_dir, "split_survey_df.pkl")
        self.original_survey_df_path = os.path.join(data_dir, "original_survey_df.pkl")
        
    def load_dataset(self, version: str = "split") -> List[Dict]:
        """加载SciReviewGen数据集"""
        if version == "split":
            file_path = self.split_survey_df_path
        else:
            file_path = self.original_survey_df_path
            
        if not os.path.exists(file_path):
            print(f"Dataset file not found: {file_path}")
            print("Please download from: https://drive.google.com/file/d/1S6v-xaCDND4ilK38sEpkfcOoMnffX7Zf/view")
            return self._load_sample_data()
        
        try:
            import pandas as pd
            df = pd.read_pickle(file_path)
            return df.to_dict('records')
        except ImportError:
            print("pandas not installed, returning sample data")
            return self._load_sample_data()
    
    def _load_sample_data(self) -> List[Dict]:
        """返回示例数据以供测试"""
        sample_data_dir = os.path.join(self.data_dir, "samples")
        samples = []
        
        if os.path.exists(sample_data_dir):
            for filename in os.listdir(sample_data_dir):
                if filename.endswith(".json"):
                    with open(os.path.join(sample_data_dir, filename), 'r', encoding='utf-8') as f:
                        samples.append(json.load(f))
        
        if not samples:
            samples = self._generate_fallback_samples()
        
        return samples
    
    def _generate_fallback_samples(self) -> List[Dict]:
        """生成回退示例数据"""
        return [
            {
                "paper_id": "sample-1",
                "title": "Large Language Models in Natural Language Processing",
                "abstract": "This survey provides a comprehensive overview of large language models...",
                "section": "Introduction",
                "text": "Large language models (LLMs) have revolutionized natural language processing...",
                "n_bibs": 15,
                "bib_titles": ["GPT-3: Language Models are Few-Shot Learners", "BERT: Pre-training of Deep Bidirectional Transformers for Language Understanding"],
                "bib_abstracts": ["GPT-3 demonstrates that language models can perform...", "BERT introduces a new pre-training approach..."],
                "split": "test"
            },
            {
                "paper_id": "sample-2",
                "title": "Deep Learning for Computer Vision",
                "abstract": "This review summarizes recent advances in deep learning for computer vision...",
                "section": "Convolutional Neural Networks",
                "text": "Convolutional Neural Networks (CNNs) have become the dominant architecture...",
                "n_bibs": 12,
                "bib_titles": ["AlexNet: ImageNet Classification with Deep Convolutional Neural Networks", "ResNet: Deep Residual Learning for Image Recognition"],
                "bib_abstracts": ["AlexNet achieved breakthrough performance on ImageNet...", "ResNet introduced residual connections..."],
                "split": "test"
            }
        ]
    
    def get_review_by_topic(self, topic: str) -> Optional[Dict]:
        """根据主题获取参考综述"""
        dataset = self.load_dataset()
        for item in dataset:
            if topic.lower() in item.get("title", "").lower():
                return item
        return None
    
    def get_all_topics(self) -> List[str]:
        """获取所有综述主题"""
        dataset = self.load_dataset()
        return list(set(item.get("title", "") for item in dataset))

class ReferenceSurvey:
    """参考综述类"""
    def __init__(self, title: str, abstract: str, sections: Dict[str, str], references: List[Dict]):
        self.title = title
        self.abstract = abstract
        self.sections = sections
        self.references = references
    
    @classmethod
    def from_dict(cls, data: Dict) -> 'ReferenceSurvey':
        """从字典创建参考综述"""
        sections = {}
        if "section" in data and "text" in data:
            sections[data["section"]] = data["text"]
        references = []
        if "bib_titles" in data and "bib_abstracts" in data:
            titles = data["bib_titles"]
            abstracts = data["bib_abstracts"]
            for i, (title, abstract) in enumerate(zip(titles, abstracts)):
                references.append({
                    "id": f"BIB{i+1:03d}",
                    "title": title,
                    "abstract": abstract
                })
        return cls(
            title=data.get("title", ""),
            abstract=data.get("abstract", ""),
            sections=sections,
            references=references
        )
    
    def get_full_text(self) -> str:
        """获取完整文本"""
        parts = [f"# {self.title}", self.abstract]
        for section_title, content in self.sections.items():
            parts.append(f"## {section_title}")
            parts.append(content)
        return "\n\n".join(parts)