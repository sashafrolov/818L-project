import glob
import yaml

def get_yaml_files():
    # Replace 'your_directory' with the path to your directory
    yaml_files = glob.glob("*.yaml")
    return yaml_files

def get_regexes_from_yaml(file_path):
    regexes = []
    with open(file_path, 'r') as file:
        regex_rules = yaml.safe_load(file)
        if regex_rules is None:
            return []
        for rule in regex_rules:
            regex = rule['rule']
            regexes.append(regex)
    return regexes

if __name__ == "__main__":
    yaml_files = get_yaml_files()

    purple_llama_regexes = []
    for file in yaml_files:
        print(file)
        regexes = get_regexes_from_yaml(file)
        purple_llama_regexes += regexes

    # As-is, this has a bug where it doesn't properly escape single quotes. Need to fix this (I did it manually)
    print('[')
    for regex in purple_llama_regexes:
        print("'" + regex + "'" + ',')
    print(']')
    