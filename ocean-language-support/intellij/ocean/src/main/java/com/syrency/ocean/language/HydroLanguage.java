package com.syrency.ocean.language;

import com.intellij.lang.Language;

public class HydroLanguage extends Language {
    public static final HydroLanguage INSTANCE = new HydroLanguage();

    private HydroLanguage() {
        super("Hydro");
    }
}