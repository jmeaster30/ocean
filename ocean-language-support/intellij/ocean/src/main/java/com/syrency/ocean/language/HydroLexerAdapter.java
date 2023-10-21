package com.syrency.ocean.language;

import com.intellij.lexer.FlexAdapter;

public class HydroLexerAdapter extends FlexAdapter {
    public HydroLexerAdapter() {
        super(new HydroLexer(null));
    }
}
