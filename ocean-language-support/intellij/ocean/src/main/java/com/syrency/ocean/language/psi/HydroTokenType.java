package com.syrency.ocean.language.psi;

import com.intellij.psi.tree.IElementType;
import com.syrency.ocean.language.HydroLanguage;
import org.jetbrains.annotations.NonNls;
import org.jetbrains.annotations.NotNull;

public class HydroTokenType extends IElementType {
    public HydroTokenType(@NotNull @NonNls String debugName) {
        super(debugName, HydroLanguage.INSTANCE);
    }

    @Override
    public String toString() {
        return "HydroTokenType." + super.toString();
    }
}
