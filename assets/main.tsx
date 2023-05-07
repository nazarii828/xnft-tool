import {
    View,
    Text,
    StyleSheet
} from "react-native";
import * as Linking from "expo-linking";
import { registerRootComponent } from "expo";

export default function App() {
    return <View style={styles.wrapper}>
        <View
            style={{
                display: "flex",
                flexDirection: "column",
                gap: 12,
                marginBottom: 24,
                opacity: 1,
            }}
        >
            <View>
                <Text style={styles.waoText}>
                    WAO
                </Text>
            </View>
            <View style={styles.imageContainer}>
                <RedBackpack height="150px" width="150px" />
            </View>
            <View>
                <Text style={styles.boxedText}>
                    get started by editing <Text style={{ fontWeight: "900" }} >main.tsx</Text>
                </Text>
                <Text
                    style={styles.boxedText}
                >
                    or check out the docs <Text style={{ fontWeight: "900" }} >docs.xnft.dev</Text>
                </Text>
            </View>
            <View style={styles.footer}>
                <Text
                    style={styles.boxedText}
                    onPress={() => Linking.openURL("https://mutuku.xyz")}
                >
                    with ❤️ from <Text style={{ fontWeight: "900" }} >jimii</Text>
                </Text>
            </View>
        </View>
    </View>
}

const styles = StyleSheet.create({
    wrapper: {
        backgroundColor: "white",
        height: "100vh",
        width: "100vw",
        padding: "12px",
        opacity: 1,
    },
    heading: {
        fontWeight: "600",
        textAlign: "center",
        marginTop: "10px"
    },
    waoText: {
        fontSize: 26,
        fontWeight: "500",
        marginBottom: 24,
        marginTop: 2,
        textAlign: "center"
    },
    imageContainer: {
        display: "flex",
        flexDirection: "row",
        justifyContent: "center",
        marginBottom: "20px",
        marginTop: "80px",
    },
    
    footer: {
        marginTop: "150px",
    },
    boxedText: {
        border: "1px solid black",
        marginTop: "10px",
        padding: "10px", 
        textAlign: "center"
    }
});

registerRootComponent(App);

function RedBackpack({ height, width }: { height: string, width: string }) {
    return (
        <svg
            xmlns="http://www.w3.org/2000/svg"
            width={width}
            height={height}
            viewBox="0 0 55 80"
            fill="none"
        >
            <path
                fillRule="evenodd"
                clipRule="evenodd"
                d="M32.71 6.29026C35.6178 6.29026 38.3452 6.68005 40.8705 7.40296C38.3982 1.64085 33.2649 0 27.5519 0C21.8277 0 16.6855 1.64729 14.2188 7.43692C16.7255 6.68856 19.4412 6.29026 22.339 6.29026H32.71ZM21.6739 12.0752C7.86677 12.0752 0 22.9371 0 36.336V50.1C0 51.4399 1.11929 52.5 2.5 52.5H52.5C53.8807 52.5 55 51.4399 55 50.1V36.336C55 22.9371 45.8521 12.0752 32.0449 12.0752H21.6739ZM27.4805 36.4551C32.313 36.4551 36.2305 32.5376 36.2305 27.7051C36.2305 22.8726 32.313 18.9551 27.4805 18.9551C22.648 18.9551 18.7305 22.8726 18.7305 27.7051C18.7305 32.5376 22.648 36.4551 27.4805 36.4551ZM0 60.5901C0 59.2503 1.11929 58.1641 2.5 58.1641H52.5C53.8807 58.1641 55 59.2503 55 60.5901V75.1466C55 77.8264 52.7614 79.9988 50 79.9988H5C2.23857 79.9988 0 77.8264 0 75.1466V60.5901Z"
                fill=" #E33E3F"
            />
        </svg>
    );
}