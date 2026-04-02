import os
import re

def process_file(filepath):
    with open(filepath, 'r', encoding='utf-8') as f:
        content = f.read()

    new_content = content.replace('SqlitePool', 'PgPool')
    new_content = new_content.replace('SqliteConnectOptions', 'PgConnectOptions')
    new_content = new_content.replace('sqlite::', 'postgres::')

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

main_path = r"e:\Carbon Credit Project\backend\src\main.rs"
if os.path.exists(main_path):
    process_file(main_path)

print("Pool substitution finished.")
