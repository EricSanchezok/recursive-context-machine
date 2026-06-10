import os
import json
import re
from typing import Dict, List, Optional
from loaders.scireviewgen_loader import SciReviewGenLoader, ReferenceSurvey

class ComparativeEvaluator:
    def __init__(self, generated_survey_path: str, reference_survey: Optional[ReferenceSurvey] = None):
        self.generated_survey_path = generated_survey_path
        self.reference_survey = reference_survey
        self.loader = SciReviewGenLoader()
    
    def load_generated_survey(self) -> str:
        if os.path.exists(self.generated_survey_path):
            with open(self.generated_survey_path, 'r', encoding='utf-8') as f:
                return f.read()
        return ""
    
    def get_reference_by_topic(self, topic: str) -> Optional[ReferenceSurvey]:
        """Fuzzy topic matching - tries multiple strategies."""
        dataset = self.load_dataset()
        if not dataset:
            return None
        
        topic_lower = topic.lower()
        words = topic_lower.split()
        
        best_match = None
        best_score = 0
        
        for item in dataset:
            title = item.get("title", "").lower()
            
            # Strategy 1: exact substring match
            if topic_lower in title:
                return ReferenceSurvey.from_dict(item)
            
            # Strategy 2: word overlap score
            title_words = set(title.split())
            overlap = len(set(words) & title_words)
            score = overlap / max(len(words), 1)
            
            if score > best_score:
                best_score = score
                best_match = item
        
        # Return best match if score is reasonable
        if best_match and best_score >= 0.3:
            print(f"  Fuzzy matched: '{topic}' -> '{best_match.get('title', '')}' (score={best_score:.2f})")
            return ReferenceSurvey.from_dict(best_match)
        
        return None
    
    def load_dataset(self) -> List[Dict]:
        """Load SciReviewGen dataset, return None if not available."""
        dataset = self.loader.load_dataset()
        if dataset and len(dataset) > 2:
            return dataset
        
        # Check if sample data exists
        sample_data_dir = "datasets/SciReviewGen/samples" if os.path.exists("datasets/SciReviewGen/samples") else None
        if sample_data_dir:
            # Load real sample JSON files
            samples = []
            for fname in os.listdir(sample_data_dir):
                if fname.endswith(".json"):
                    with open(os.path.join(sample_data_dir, fname)) as f:
                        samples.append(json.load(f))
            if samples:
                return samples
        
        # Return the fallback samples
        return self.loader._generate_fallback_samples()
    
    def extract_key_points(self, text: str) -> List[str]:
        """Extract key points with better sentence splitting."""
        key_points = []
        
        # Split by markdown headings
        sections = re.split(r'(?:^|\n)(#{1,3})\s+', text, flags=re.MULTILINE)
        
        current_level = ""
        current_title = ""
        
        for i, part in enumerate(sections):
            part = part.strip()
            if not part:
                continue
            
            if i % 2 == 1:
                # This is a heading level marker
                current_level = part
                continue
            elif i % 2 == 0 and current_level:
                # This is heading title + content
                lines = part.split('\n')
                current_title = lines[0].strip()
                content = '\n'.join(lines[1:])
            else:
                # First chunk before any heading
                content = part
                current_title = "Preamble"
            
            # Extract sentences from content
            sentences = re.split(r'(?<=[.!?])\s+', content)
            for sentence in sentences:
                sentence = sentence.strip()
                # Remove bullet markers, bold markers, etc.
                sentence = re.sub(r'^[-*\s]+', '', sentence)
                sentence = re.sub(r'\*\*([^*]+)\*\*', r'\1', sentence)
                if len(sentence) > 40 and len(sentence) < 500:
                    key_points.append(f"[{current_title}] {sentence[:200]}")
        
        return key_points[:30]
    
    def compare_key_points(self) -> Dict:
        if not self.reference_survey:
            return {"error": "No reference survey provided"}
        
        generated_text = self.load_generated_survey()
        reference_text = self.reference_survey.get_full_text()
        
        generated_points = self.extract_key_points