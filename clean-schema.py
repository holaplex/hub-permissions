import re
import sys

def remove_comments(file_path):
    with open(file_path, 'r') as f:
        content = f.read()

    # Regex pattern to match triple double quotes (""") followed by any characters (including newlines) and ending with triple double quotes (""")
    pattern = r'"""[\s\S]*?"""'

    # Remove comments
    content_no_comments = re.sub(pattern, '', content)

    # Remove extra newlines
    content_compact = re.sub(r'\n\s*\n', '\n', content_no_comments)

    return content_compact

if __name__ == "__main__":
    if len(sys.argv) != 2:
        print("Usage: python3 remove_comments.py <path_to_schema_file>")
        sys.exit(1)

    file_path = sys.argv[1]
    result = remove_comments(file_path)
    print(result)
