pub struct Yacc<'input> {
    pub declarations: Vec<BisonDecl<'input>>,
    pub rules: Vec<GrammarRule<'input>>
}

pub enum BisonDecl<'input> {
    Token(Ident<'input>),
    Start(Ident<'input>),
}

pub struct GrammarRule<'input> {
    pub nonterminal: Ident<'input>,
    pub alternatives: Vec<Alternative<'input>>,
}

pub struct Alternative<'input> {
    pub symbols: Vec<Symbol<'input>>
}

pub enum Symbol<'input> {
    Ident(Ident<'input>),
    Character(&'input str),
}

#[derive(Copy, Clone, PartialEq, Eq)]
pub struct Ident<'input> {
    pub text: &'input str
}
