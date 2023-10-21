package com.syrency.ocean.language.psi;

import com.intellij.psi.tree.TokenSet;

public interface HydroTokenSets {
    TokenSet BASETYPES = TokenSet.create(HydroTypes.BASETYPE);
    TokenSet IDENTIFIERS = TokenSet.create(HydroTypes.IDENTIFIER);
    TokenSet COMMENTS = TokenSet.create(HydroTypes.COMMENT);
    TokenSet NUMBERS = TokenSet.create(HydroTypes.NUMBER);
    TokenSet STRING = TokenSet.create(HydroTypes.STRING);
    TokenSet KEYWORDS = TokenSet.create(HydroTypes.KEYWORD);
}
