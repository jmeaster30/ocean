package com.syrency.ocean.language.psi;

import com.intellij.psi.tree.IElementType;
import com.syrency.ocean.language.HydroLanguage;
import org.jetbrains.annotations.NonNls;
import org.jetbrains.annotations.NotNull;

public class HydroElementType extends IElementType {
    public HydroElementType(@NotNull @NonNls String debugName) {
        super(debugName, HydroLanguage.INSTANCE);
    }
}
