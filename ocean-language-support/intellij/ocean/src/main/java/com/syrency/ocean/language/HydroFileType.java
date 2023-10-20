package com.syrency.ocean.language;

import com.intellij.openapi.fileTypes.LanguageFileType;
import org.jetbrains.annotations.NotNull;
import org.jetbrains.annotations.Nullable;

import javax.swing.Icon;

public class HydroFileType extends LanguageFileType {
    public static final HydroFileType INSTANCE = new HydroFileType();

    private HydroFileType() {
        super(HydroLanguage.INSTANCE);
    }

    @NotNull
    @Override
    public String getName() {
        return "Hydro Source File";
    }

    @NotNull
    @Override
    public String getDescription() {
        return "Hydro source file";
    }

    @NotNull
    @Override
    public String getDefaultExtension() {
        return "h2o";
    }

    @Nullable
    @Override
    public Icon getIcon() {
        return HydroIcons.BASE;
    }
}
