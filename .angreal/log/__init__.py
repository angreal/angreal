class log:
    f = lambda color: lambda string: print(color + string + "\33[0m")

    black = f("\33[30m")
    red = f("\33[31m")
    green = f("\33[32m")
    yellow = f("\33[33m")
    blue = f("\33[34m")
    magenta = f("\33[35m")
    cyan = f("\33[36m")
    white = f("\33[37m")

# Usage
log.blue("Blue World!")