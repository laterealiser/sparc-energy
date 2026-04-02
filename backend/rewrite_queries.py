import os
import re

def convert_to_postgres_binds(match):
    text = match.group(0)
    counter = 1
    def replace_question_mark(m):
        nonlocal counter
        res = f"${counter}"
        counter += 1
        return res
    return re.sub(r'\?', replace_question_mark, text)

def process_file(filepath):
    with open(filepath, 'r', encoding='utf-8') as f:
        content = f.read()

    # Find string literals:
    # We look for r##"..."## or r#"..."# or "..."
    # A simple approach for standard "..." strings and raw strings.
    
    # We can match `r?#*"(?:[^"\\]|\\.)*?"#*` roughly, but since we know our queries are standard `"...""`,
    # we can just find any string literal that has a `?` inside it and is likely SQL
    
    def replacer(m):
        s = m.group(0)
        if "?" in s and ("SELECT" in s or "UPDATE" in s or "INSERT" in s or "DELETE" in s or "WHERE" in s):
            return convert_to_postgres_binds(m)
        return s

    # Regex to match strings: "..."
    # Handles escapes \"
    new_content = re.sub(r'"([^"\\]|\\.)*"', replacer, content)

    # Some raw strings: r#"..."#
    new_content = re.sub(r'r#"([\s\S]*?)"#', replacer, new_content)

    if new_content != content:
        with open(filepath, 'w', encoding='utf-8') as f:
            f.write(new_content)
        print(f"Updated {filepath}")

src_dir = r"e:\Carbon Credit Project\backend\src\handlers"
for filename in os.listdir(src_dir):
    if filename.endswith(".rs"):
        process_file(os.path.join(src_dir, filename))

db_path = r"e:\Carbon Credit Project\backend\src\db.rs"
if os.path.exists(db_path):
    process_file(db_path)

print("Substitution script finished.")
