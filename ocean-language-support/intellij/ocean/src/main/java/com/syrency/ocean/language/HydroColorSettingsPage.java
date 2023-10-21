package com.syrency.ocean.language;

import com.intellij.openapi.editor.colors.TextAttributesKey;
import com.intellij.openapi.fileTypes.SyntaxHighlighter;
import com.intellij.openapi.options.colors.AttributesDescriptor;
import com.intellij.openapi.options.colors.ColorDescriptor;
import com.intellij.openapi.options.colors.ColorSettingsPage;
import com.intellij.openapi.util.NlsContexts;
import org.jetbrains.annotations.NonNls;
import org.jetbrains.annotations.NotNull;
import org.jetbrains.annotations.Nullable;

import javax.swing.*;
import java.util.Map;

public class HydroColorSettingsPage implements ColorSettingsPage {

    private static final AttributesDescriptor[] DESCRIPTORS = new AttributesDescriptor[]{
        new AttributesDescriptor("Bad characters", HydroSyntaxHighlighter.BAD_CHARACTER),
        new AttributesDescriptor("Types", HydroSyntaxHighlighter.BASETYPE),
        new AttributesDescriptor("Comments", HydroSyntaxHighlighter.COMMENT),
        new AttributesDescriptor("Identifiers", HydroSyntaxHighlighter.IDENTIFIER),
        new AttributesDescriptor("Keywords", HydroSyntaxHighlighter.KEYWORD),
        new AttributesDescriptor("Numbers", HydroSyntaxHighlighter.NUMBER),
        new AttributesDescriptor("Strings", HydroSyntaxHighlighter.STRING),
    };

    @Override
    public @Nullable Icon getIcon() {
        return HydroIcons.BASE;
    }

    @Override
    public @NotNull SyntaxHighlighter getHighlighter() {
        return new HydroSyntaxHighlighter();
    }

    @Override
    public @NonNls @NotNull String getDemoText() {
        return  "% This is a demo to show off the syntax highlighting!!\n" +
                "module main\n" +
                "layout point\n" +
                "\ts128 x\n" +
                "\ts128 y\n\n" +
                "using another_module\n\n" +
                "function fibonacci any body\n" +
                "\tduplicate\n" +
                "\tduplicate\n" +
                "\tpush u128 1\n" +
                "\tlessthanequal\n" +
                "\tpush bool true\n" +
                "\tequal\n" +
                "\tbranch finish notfinish\n" +
                "\tlabel finish\n" +
                "\treturn\n" +
                "\tlabel notfinish\n" +
                "\tpush u128 1\n" +
                "\tsubtract\n" +
                "\tpush funcp main fibonacci\n" +
                "\tcall\n" +
                "\tswap\n" +
                "\tpush u128 2\n" +
                "\tsubtract\n" +
                "\tpush funcp main fibonacci\n" +
                "\tcall\n" +
                "\tadd\n" +
                "\treturn\n\n" +
                "main body\n" +
                "\tpush string \"example string :)\"" +
                "\tpush u128 20\n" +
                "\tpush funcp main fibonacci\n" +
                "\tcall\n" +
                "\treturn\n";
    }

    @Override
    public @Nullable Map<String, TextAttributesKey> getAdditionalHighlightingTagToDescriptorMap() {
        return null;
    }

    @Override
    public AttributesDescriptor @NotNull [] getAttributeDescriptors() {
        return DESCRIPTORS;
    }

    @Override
    public ColorDescriptor @NotNull [] getColorDescriptors() {
        return ColorDescriptor.EMPTY_ARRAY;
    }

    @Override
    public @NotNull @NlsContexts.ConfigurableName String getDisplayName() {
        return "Hydro";
    }
}
