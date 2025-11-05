"""
Comprehensive Text Processing Example
Combines: string, re, collections, operator

This example demonstrates realistic text processing scenarios using
multiple Python stdlib modules working together.

Tests transpiler's ability to handle:
- String operations with collections
- Pattern matching with data structures
- Text analysis pipelines
- Character classification with counting
"""

import string
from collections import Counter, defaultdict
from typing import List, Dict, Tuple


def tokenize_text(text: str) -> List[str]:
    """Tokenize text into words using string operations"""
    # Remove punctuation
    cleaned: str = ""
    punctuation: str = ".,!?;:\"'()[]{}—-"

    for char in text:
        is_punct: bool = False
        for p in punctuation:
            if char == p:
                is_punct = True
                break

        if not is_punct:
            cleaned = cleaned + char
        else:
            cleaned = cleaned + " "

    # Split into words
    words: List[str] = cleaned.split()

    # Normalize to lowercase
    normalized: List[str] = []
    for word in words:
        normalized.append(word.lower())

    return normalized


def count_word_frequencies(words: List[str]) -> Dict[str, int]:
    """Count word frequencies using Counter pattern"""
    frequencies: Dict[str, int] = {}

    for word in words:
        if word in frequencies:
            frequencies[word] = frequencies[word] + 1
        else:
            frequencies[word] = 1

    return frequencies


def get_most_common_words(frequencies: Dict[str, int], n: int) -> List[Tuple[str, int]]:
    """Get n most common words"""
    # Convert to list of tuples
    word_counts: List[Tuple[str, int]] = []
    for word in frequencies.keys():
        count: int = frequencies[word]
        word_counts.append((word, count))

    # Sort by count (descending)
    for i in range(len(word_counts)):
        for j in range(i + 1, len(word_counts)):
            if word_counts[j][1] > word_counts[i][1]:
                temp: Tuple[str, int] = word_counts[i]
                word_counts[i] = word_counts[j]
                word_counts[j] = temp

    # Return top n
    result: List[Tuple[str, int]] = []
    for i in range(min(n, len(word_counts))):
        result.append(word_counts[i])

    return result


def analyze_character_distribution(text: str) -> Dict[str, int]:
    """Analyze character types using string module patterns"""
    distribution: Dict[str, int] = {
        "letters": 0,
        "digits": 0,
        "spaces": 0,
        "punctuation": 0,
        "other": 0
    }

    for char in text:
        if char.isalpha():
            distribution["letters"] = distribution["letters"] + 1
        elif char.isdigit():
            distribution["digits"] = distribution["digits"] + 1
        elif char.isspace():
            distribution["spaces"] = distribution["spaces"] + 1
        elif char in ".,!?;:":
            distribution["punctuation"] = distribution["punctuation"] + 1
        else:
            distribution["other"] = distribution["other"] + 1

    return distribution


def extract_sentences(text: str) -> List[str]:
    """Extract sentences using simple pattern matching"""
    sentences: List[str] = []
    current_sentence: str = ""

    for char in text:
        current_sentence = current_sentence + char

        # End of sentence markers
        if char == "." or char == "!" or char == "?":
            # Trim whitespace
            trimmed: str = current_sentence.strip()
            if len(trimmed) > 0:
                sentences.append(trimmed)
            current_sentence = ""

    # Add remaining text
    if len(current_sentence.strip()) > 0:
        sentences.append(current_sentence.strip())

    return sentences


def calculate_readability_metrics(text: str) -> Dict[str, float]:
    """Calculate readability metrics combining multiple operations"""
    metrics: Dict[str, float] = {}

    # Tokenize
    words: List[str] = tokenize_text(text)
    sentences: List[str] = extract_sentences(text)

    # Word count
    metrics["word_count"] = float(len(words))

    # Sentence count
    metrics["sentence_count"] = float(len(sentences))

    # Average word length
    total_chars: int = 0
    for word in words:
        total_chars = total_chars + len(word)

    if len(words) > 0:
        metrics["avg_word_length"] = float(total_chars) / float(len(words))
    else:
        metrics["avg_word_length"] = 0.0

    # Average sentence length
    if len(sentences) > 0:
        metrics["avg_sentence_length"] = float(len(words)) / float(len(sentences))
    else:
        metrics["avg_sentence_length"] = 0.0

    return metrics


def group_words_by_length(words: List[str]) -> Dict[int, List[str]]:
    """Group words by length using collections pattern"""
    groups: Dict[int, List[str]] = {}

    for word in words:
        length: int = len(word)

        if length not in groups:
            groups[length] = []

        groups[length].append(word)

    return groups


def find_word_patterns(words: List[str]) -> Dict[str, List[str]]:
    """Find words matching patterns (starts with, ends with, contains)"""
    patterns: Dict[str, List[str]] = {
        "starts_with_a": [],
        "ends_with_ing": [],
        "contains_th": []
    }

    for word in words:
        # Starts with 'a'
        if len(word) > 0 and word[0] == "a":
            patterns["starts_with_a"].append(word)

        # Ends with 'ing'
        if len(word) >= 3 and word[-3:] == "ing":
            patterns["ends_with_ing"].append(word)

        # Contains 'th'
        if "th" in word:
            patterns["contains_th"].append(word)

    return patterns


def create_ngrams(words: List[str], n: int) -> List[str]:
    """Create n-grams from word list"""
    ngrams: List[str] = []

    for i in range(len(words) - n + 1):
        ngram_words: List[str] = []
        for j in range(n):
            ngram_words.append(words[i + j])

        ngram: str = " ".join(ngram_words)
        ngrams.append(ngram)

    return ngrams


def calculate_word_diversity(words: List[str]) -> float:
    """Calculate lexical diversity (unique words / total words)"""
    if len(words) == 0:
        return 0.0

    # Count unique words
    unique_words: Dict[str, bool] = {}
    for word in words:
        unique_words[word] = True

    diversity: float = float(len(unique_words)) / float(len(words))
    return diversity


def find_palindromes(words: List[str]) -> List[str]:
    """Find palindrome words"""
    palindromes: List[str] = []

    for word in words:
        # Reverse word
        reversed_word: str = ""
        for i in range(len(word) - 1, -1, -1):
            reversed_word = reversed_word + word[i]

        if word == reversed_word and len(word) > 1:
            # Check if already in list
            found: bool = False
            for p in palindromes:
                if p == word:
                    found = True
                    break

            if not found:
                palindromes.append(word)

    return palindromes


def analyze_vowel_consonant_ratio(text: str) -> Dict[str, float]:
    """Analyze vowel to consonant ratio"""
    vowels: str = "aeiouAEIOU"
    vowel_count: int = 0
    consonant_count: int = 0

    for char in text:
        if char.isalpha():
            if char in vowels:
                vowel_count = vowel_count + 1
            else:
                consonant_count = consonant_count + 1

    total_letters: int = vowel_count + consonant_count

    result: Dict[str, float] = {}
    if total_letters > 0:
        result["vowel_ratio"] = float(vowel_count) / float(total_letters)
        result["consonant_ratio"] = float(consonant_count) / float(total_letters)
    else:
        result["vowel_ratio"] = 0.0
        result["consonant_ratio"] = 0.0

    result["vowel_count"] = float(vowel_count)
    result["consonant_count"] = float(consonant_count)

    return result


def process_text_pipeline() -> None:
    """Main text processing pipeline"""
    print("=== Comprehensive Text Processing Demo ===")

    # Sample text
    sample_text: str = """
    The quick brown fox jumps over the lazy dog. This is a sample text
    for demonstrating text processing capabilities. Python is amazing!
    We can analyze words, count frequencies, and find patterns easily.
    """

    # Tokenize
    words: List[str] = tokenize_text(sample_text)
    print(f"Total words: {len(words)}")

    # Frequency analysis
    frequencies: Dict[str, int] = count_word_frequencies(words)
    print(f"Unique words: {len(frequencies)}")

    # Most common words
    top_words: List[Tuple[str, int]] = get_most_common_words(frequencies, 5)
    print(f"Top 5 words: {len(top_words)}")

    # Character distribution
    char_dist: Dict[str, int] = analyze_character_distribution(sample_text)
    print(f"Letters: {char_dist['letters']}, Digits: {char_dist['digits']}")

    # Sentences
    sentences: List[str] = extract_sentences(sample_text)
    print(f"Sentences: {len(sentences)}")

    # Readability metrics
    metrics: Dict[str, float] = calculate_readability_metrics(sample_text)
    print(f"Avg word length: {metrics['avg_word_length']:.2f}")

    # Group by length
    length_groups: Dict[int, List[str]] = group_words_by_length(words)
    print(f"Length groups: {len(length_groups)}")

    # Find patterns
    patterns: Dict[str, List[str]] = find_word_patterns(words)
    print(f"Words starting with 'a': {len(patterns['starts_with_a'])}")

    # Create bigrams
    bigrams: List[str] = create_ngrams(words, 2)
    print(f"Bigrams created: {len(bigrams)}")

    # Calculate diversity
    diversity: float = calculate_word_diversity(words)
    print(f"Lexical diversity: {diversity:.2f}")

    # Find palindromes
    palindromes: List[str] = find_palindromes(words)
    print(f"Palindromes found: {len(palindromes)}")

    # Vowel/consonant ratio
    ratios: Dict[str, float] = analyze_vowel_consonant_ratio(sample_text)
    print(f"Vowel ratio: {ratios['vowel_ratio']:.2f}")

    print("=== Processing Complete ===")


if __name__ == "__main__":
    process_text_pipeline()
