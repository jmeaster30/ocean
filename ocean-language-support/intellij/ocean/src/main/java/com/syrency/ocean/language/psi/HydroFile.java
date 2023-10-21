package com.syrency.ocean.language.psi;

import com.intellij.extapi.psi.PsiFileBase;
import com.intellij.openapi.fileTypes.FileType;
import com.intellij.psi.FileViewProvider;
import com.syrency.ocean.language.HydroFileType;
import com.syrency.ocean.language.HydroLanguage;
import org.jetbrains.annotations.NotNull;

public class HydroFile extends PsiFileBase {
    public HydroFile(@NotNull FileViewProvider viewProvider) {
        super(viewProvider, HydroLanguage.INSTANCE);
    }

    @NotNull
    @Override
    public FileType getFileType() {
        return HydroFileType.INSTANCE;
    }

    @Override
    public String toString() {
        return "Hydro File";
    }
}
