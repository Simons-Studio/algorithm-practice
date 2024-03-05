stack = []
head = None
stack_min = None
stack_min_count = 0

def pop():
    if head == None:
        return None
    else:
        result = stack[head]
        if result == stack_min:
            if stack_min_count > 0:
                stack_min_count -= 1
            else:
                return None
            if stack_min_count == 0:
                (stack_min, stack_min_count) = get_stack_min(stack)
        if head > 0:
            head -= 1
        else:
            head = None
        return result

def push(number):
    if head == None:
        stack.append(number)
        head = 0
        stack_min = number
        stack_min_count = 1
    else:
        stack.append(number)
        head += 1
        if number == stack_min:
            stack_min_count += 1
        if number < stack_min:
            stack_min = number
            stack_min_count = 1

def get_stack_min(stack):
    new_stack_min = stack_min
    new_stack_min_count = 0
    for e in stack:
        if new_stack_min == None:
            new_stack_min = e
            new_stack_min_count = 1
        if e < new_stack_min:
            new_stack_min = e
            new_stack_min_count = 1
        if e == new_stack_min:
            new_stack_min_count += 1
    
    return (new_stack_min, new_stack_min_count)
