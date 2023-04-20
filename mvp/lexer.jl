@enum TokenKind begin
    LiteralInt
    LiteralFloat
    Word
end

struct Lexem
    kind::TokenKind
    s::Int32
    e::Int32
end

mutable struct Lexer
    input::String
    index::Int32
end

validlexem(lexem::Lexem) = begin
    if lexem.s >= lexem.e
        return missing
    end
    return lexem
end

isint(x::String) = isa(tryparse(Int, x), Number)
isfloat(x::String) = isa(tryparse(Float32, x), Number)

match_tokenkind(text::String)::TokenKind = begin
    if isint(text)
        return LiteralInt
    elseif isfloat(text)
        return LiteralFloat
    end
    return Word
end

function next(self::Lexer)::Union{Lexem, Missing}
    if self.index > length(self.input)
        return missing
    end

    s = self.index
    while !isspace(self.input[self.index]) && self.index < length(self.input)
        self.index += 1
    end
    e = self.index == length(self.input) ? self.index : self.index - 1
    self.index += 1
    
    return Lexem(match_tokenkind(self.input[s:e]), s, e)
end
