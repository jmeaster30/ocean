package com.syrency.ocean.language;

import com.intellij.lexer.Lexer;
import com.intellij.openapi.editor.DefaultLanguageHighlighterColors;
import com.intellij.openapi.editor.HighlighterColors;
import com.intellij.openapi.editor.colors.TextAttributesKey;
import com.intellij.openapi.fileTypes.SyntaxHighlighterBase;
import com.intellij.psi.TokenType;
import com.intellij.psi.tree.IElementType;
import com.syrency.ocean.language.psi.HydroTypes;
import org.jetbrains.annotations.NotNull;

import static com.intellij.openapi.editor.colors.TextAttributesKey.createTextAttributesKey;

public class HydroSyntaxHighlighter extends SyntaxHighlighterBase {
    public static final TextAttributesKey BAD_CHARACTER = createTextAttributesKey("HYDRO_BAD_CHARACTER", HighlighterColors.BAD_CHARACTER);
    public static final TextAttributesKey BASETYPE = createTextAttributesKey("HYDRO_BASETYPE", DefaultLanguageHighlighterColors.CLASS_NAME);
    public static final TextAttributesKey COMMENT = createTextAttributesKey("HYDRO_COMMENT", DefaultLanguageHighlighterColors.LINE_COMMENT);
    public static final TextAttributesKey IDENTIFIER = createTextAttributesKey("HYDRO_IDENTIFIER", DefaultLanguageHighlighterColors.IDENTIFIER);
    public static final TextAttributesKey KEYWORD = createTextAttributesKey("HYDRO_KEYWORD", DefaultLanguageHighlighterColors.KEYWORD);
    public static final TextAttributesKey NUMBER = createTextAttributesKey("HYDRO_NUMBER", DefaultLanguageHighlighterColors.NUMBER);
    public static final TextAttributesKey STRING = createTextAttributesKey("HYDRO_STRING", DefaultLanguageHighlighterColors.STRING);

    private static final TextAttributesKey[] BAD_CHAR_KEYS = new TextAttributesKey[]{BAD_CHARACTER};
    private static final TextAttributesKey[] BASETYPE_KEYS = new TextAttributesKey[]{BASETYPE};
    private static final TextAttributesKey[] COMMENT_KEYS = new TextAttributesKey[]{COMMENT};
    private static final TextAttributesKey[] EMPTY_KEYS = new TextAttributesKey[0];
    private static final TextAttributesKey[] IDENTIFIER_KEYS = new TextAttributesKey[]{IDENTIFIER};
    private static final TextAttributesKey[] KEYWORD_KEYS = new TextAttributesKey[]{KEYWORD};
    private static final TextAttributesKey[] NUMBER_KEYS = new TextAttributesKey[]{NUMBER};
    private static final TextAttributesKey[] STRING_KEYS = new TextAttributesKey[]{STRING};

    @NotNull
    @Override
    public Lexer getHighlightingLexer() {
        return new HydroLexerAdapter();
    }

    @Override
    public TextAttributesKey @NotNull [] getTokenHighlights(IElementType tokenType) {
        if (tokenType.equals(TokenType.BAD_CHARACTER)) {
            return BAD_CHAR_KEYS;
        }
        if (tokenType.equals(HydroTypes.BASETYPE)) {
            return BASETYPE_KEYS;
        }
        if (tokenType.equals(HydroTypes.COMMENT)) {
            return COMMENT_KEYS;
        }
        if (tokenType.equals(HydroTypes.IDENTIFIER)) {
            return IDENTIFIER_KEYS;
        }
        if (tokenType.equals(HydroTypes.KEYWORD)) {
            return KEYWORD_KEYS;
        }
        if (tokenType.equals(HydroTypes.STRING)) {
            return STRING_KEYS;
        }
        if (tokenType.equals(HydroTypes.BOOLEAN)) {
            return NUMBER_KEYS;
        }
        if (tokenType.equals(HydroTypes.NUMBER)) {
            return NUMBER_KEYS;
        }
        return EMPTY_KEYS;
    }
}
