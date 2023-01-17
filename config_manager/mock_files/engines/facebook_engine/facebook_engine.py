#TODO implement a better engine example
import sys

if __name__ == "__main__":
    if len(sys.argv) != 3:
        sys.exit(1)
    
    if sys.argv[1] == '-search_user' and sys.argv[2] =='user123':
        print("test output")
        sys.exit(0)
