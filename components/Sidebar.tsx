import { Project } from "@/constants/Project";
import { colorScheme, defaultFont, defaultFontBold, defaultFontItalic, fontSize } from "@/constants/style";
import React from "react";
import { Image, Pressable, Text, View } from "react-native";

export function CustomDrawerContent(safeAreaInsets: any) {
    return (
        <View style={{
            backgroundColor:colorScheme.secondary,
            height:'100%',
            paddingTop:safeAreaInsets.top,
            paddingBottom:safeAreaInsets.bottom,
            paddingHorizontal: 20
        }}>
            
        </View>
    );
}