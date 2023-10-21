package com.syrency.ocean.language;

import com.intellij.lang.ASTNode;
import com.intellij.lang.ParserDefinition;
import com.intellij.lang.PsiParser;
import com.intellij.lexer.Lexer;
import com.intellij.openapi.project.Project;
import com.intellij.psi.FileViewProvider;
import com.intellij.psi.PsiElement;
import com.intellij.psi.PsiFile;
import com.intellij.psi.tree.IFileElementType;
import com.intellij.psi.tree.TokenSet;
import com.syrency.ocean.language.parser.HydroParser;
import com.syrency.ocean.language.psi.HydroFile;
import com.syrency.ocean.language.psi.HydroTokenSets;
import com.syrency.ocean.language.psi.HydroTypes;
import org.jetbrains.annotations.NotNull;

public class HydroParserDefinition implements ParserDefinition {

    public static final IFileElementType FILE = new IFileElementType(HydroLanguage.INSTANCE);

    @Override
    public @NotNull Lexer createLexer(Project project) {
        return new HydroLexerAdapter();
    }

    @Override
    public @NotNull PsiParser createParser(Project project) {
        return new HydroParser();
    }

    @Override
    public @NotNull IFileElementType getFileNodeType() {
        return FILE;
    }

    @Override
    public @NotNull TokenSet getCommentTokens() {
        return HydroTokenSets.COMMENTS;
    }

    @Override
    public @NotNull TokenSet getStringLiteralElements() {
        return HydroTokenSets.STRING;
    }

    @Override
    public @NotNull PsiElement createElement(ASTNode node) {
        return HydroTypes.Factory.createElement(node);
    }

    @Override
    public @NotNull PsiFile createFile(@NotNull FileViewProvider viewProvider) {
        return new HydroFile(viewProvider);
    }
}
