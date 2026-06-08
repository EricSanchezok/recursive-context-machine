"""Extract text from PDF files in the pdfs/ directory and save as text files."""
import sys
import os

# Try various PDF extraction libraries
extracted = False
text = ""

pdf_path = sys.argv[1] if len(sys.argv) > 1 else None
if not pdf_path:
    print("Usage: python extract_pdf_text.py <pdf_path>")
    sys.exit(1)

# Try PyMuPDF
try:
    import fitz
    doc = fitz.open(pdf_path)
    for page in doc:
        text += page.get_text()
    extracted = True
except ImportError:
    pass

# Try pdfminer
if not extracted:
    try:
        from pdfminer.high_level import extract_text
        text = extract_text(pdf_path)
        extracted = True
    except ImportError:
        pass

# Try PyPDF2 / pypdf
if not extracted:
    try:
        import PyPDF2
        with open(pdf_path, 'rb') as f:
            reader = PyPDF2.PdfReader(f)
            for page in reader.pages:
                text += page.extract_text()
        extracted = True
    except ImportError:
        pass

# Try pypdf
if not extracted:
    try:
        import pypdf
        reader = pypdf.PdfReader(pdf_path)
        for page in reader.pages:
            text += page.extract_text()
        extracted = True
    except ImportError:
        pass

# Try pdfplumber
if not extracted:
    try:
        import pdfplumber
        with pdfplumber.open(pdf_path) as pdf:
            for page in pdf.pages:
                text += page.extract_text() + "\n"
        extracted = True
    except ImportError:
        pass

if extracted and text.strip():
    sys.stdout.write(text)
    sys.exit(0)
else:
    # Output raw bytes that look like text
    with open(pdf_path, 'rb') as f:
        raw = f.read()
    # Try to find text between parentheses (PDF text objects)
    import re
    texts = re.findall(rb'\(([^)]*)\)', raw)
    decoded = []
    for t in texts:
        try:
            decoded.append(t.decode('latin-1'))
        except:
            pass
    result = '\n'.join(decoded)
    if result.strip():
        sys.stdout.write(result)
    else:
        print("NO_TEXT_EXTRACTED")
    sys.exit(0)
