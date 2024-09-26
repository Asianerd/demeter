import { Button, Image, Pressable, ScrollView, StyleSheet, Text, View } from "react-native";
import { DefaultContainer, colorScheme, defaultFont, defaultFontBold, defaultFontItalic, fontSize } from "../constants/style";
import React, { useEffect } from "react";

import GLOBALS from './global';
import { screenSize } from "./_layout";
import { Task } from "@/constants/Task";
import { DrawerActions } from "@react-navigation/native";

function HyperlinkHeader({header}: {header: any}) {
    return (
        <View style={{
            marginTop: 10,
            marginBottom: 10
        }}>
            <Text style={{ color:colorScheme.primary, fontFamily: defaultFontItalic, fontSize:fontSize.tiny * 1.3 }}>
                {header}
            </Text>
        </View>
    );
}

function Home({navigation, route}: {navigation: any, route: any}) {
    return <DefaultContainer menu="home">
        
    </DefaultContainer>
}

export default Home;