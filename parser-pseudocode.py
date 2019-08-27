def parse_program(tokens):
    function = parse_function(tokens)
    program = Program(function)

    return program

def parse_function(tokens):
    tok = tokens.next()
    if tok.type != "INT_KEYWORD":
        fail()
    
    tok = tokens.next()
    if tok.type != "IDENTIFIER":
        fail()
    
    tok = tokens.next()
    if tok.type != "OPEN_PAREN":
        fail()

    tok = tokens.next()
    if tok.type != "CLOSE_PAREN":
        fail()
    
    tok = tokens.next()
    if tok.type != "OPEN_BRACE":
        fail()

    statement = parse_statement(tokens)
    function = Function(statement)

    tok = tokens.next()
    if tok.type != "CLOSE_BRACE":
        fail()
    
    return function

def parse_statement(tokens):
    tok = tokens.next()
    if tok.type != "RETURN_KEYWORD":
        fail()

    tok = tokens.next()
    if tok.type != "INT":
        fail()
    
    exp = parse_exp(tokens)
    statement = Return(exp)

    tok = tokens.next()
    if tok.type != "SEMICOLON":
        fail()
    
    return statement

def parse_exp(tokens):
    tok = tokens.next()
    if tok.type != "INT":
        fail()

    val = tok.value.parse_int()
    return Expression(val)