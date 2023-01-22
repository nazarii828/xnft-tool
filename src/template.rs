// ! generating the config files

pub struct RNTemplate {
    //
}

impl RNTemplate {
    pub fn webpack_config_js() -> &'static str {
        r#"{
      const createExpoWebpackConfigAsync = require("@expo/webpack-config");
      const ReactRefreshWebpackPlugin = require("@pmmmwh/react-refresh-webpack-plugin");
      
      const fs = require("fs");
      
      module.exports = async function (env, argv) {
        const config = await createExpoWebpackConfigAsync(env, argv);
      
        if (env.mode === "development") {
          config.plugins.push(new ReactRefreshWebpackPlugin());
          // keep everything else the same for expo start
          return config;
        }
      
        config.output = {
          globalObject: "this",
          path: __dirname + "/dist/.artifacts/",
          filename: "index.js",
        };
      
        config.optimization.splitChunks = {
          cacheGroups: {
            default: false,
          },
        };
        config.optimization.runtimeChunk = false;
      
        config.plugins = config.plugins.filter((plugin) =>
          ["DefinePlugin", "CleanWebpackPlugin"].includes(plugin.constructor.name)
        );
      
        config.plugins.push(
          new InlineJSPlugin({
            template: "template.html",
            filename: "index.html",
          })
        );
      
        // this is brittle but works for now.
        const loaders = config.module.rules.find(
          (rule) => typeof rule.oneOf !== "undefined"
        );
        const urlLoader = loaders.oneOf.find(
          (loader) =>
            typeof loader.use === "object" &&
            loader.use.loader &&
            loader.use.loader.includes("url-loader")
        );
      
        urlLoader.use.options.limit = true;
        urlLoader.test = /\.(gif|jpe?g|png|svg|css|woff2?|eot|ttf|otf)$/;
      
        return config;
      };
      
      // const logger = console.log.bind(console);
      
      class InlineJSPlugin {
        constructor({ template, filename }) {
          this.options = {
            template,
            filename,
          };
        }
        apply(compiler) {
          compiler.hooks.done.tap("InlineJSPlugin", (stats) => {
            const filename = stats.compilation.outputOptions.filename;
            const path = stats.compilation.outputOptions.path;
            const asset = stats.compilation.assets[filename];
            const JSBundle = asset.children[0]._value;
            const template = fs
              .readFileSync(this.options.template)
              .toString()
              .split(" ####JS####");
            fs.writeFileSync(
              path + "/../" + this.options.filename,
              template[0] + JSBundle + template[1]
            );
          });
        }
      }
      
    }"#
    }

    pub fn ts_config() -> &'static str {
        r#"{
        "extends": "expo/tsconfig.base",
        "compilerOptions": {
          "strict": true
        }
      }
    "#
    }

    pub fn template_html() -> &'static str {
        r#"
  <!doctype html>
  <html lang="en">
  
  <head>
	  <meta charset="utf-8" />
	  <meta httpequiv="X-UA-Compatible" content="IE=edge" />
	  <meta name="viewport"
		  content="width=device-width,initial-scale=1,minimum-scale=1,maximum-scale=1.00001,viewport-fit=cover" />
	  <title>
		  xNFT
	  </title>
	  <style>
		  #root,
		  body,
		  html {
			  width: 100%;
			  -webkit-overflow-scrolling: touch;
			  margin: 0;
			  padding: 0;
			  min-height: 100%
		  }
  
		  #root {
			  flex-shrink: 0;
			  flex-basis: auto;
			  flex-grow: 1;
			  display: flex;
			  flex: 1
		  }
  
		  html {
			  scroll-behavior: smooth;
			  -webkit-text-size-adjust: 100%;
			  height: calc(100% + env(safe-area-inset-top))
		  }
  
		  body {
			  display: flex;
			  overflow-y: auto;
			  overscroll-behavior-y: none;
			  text-rendering: optimizeLegibility;
			  -webkit-font-smoothing: antialiased;
			  -moz-osx-font-smoothing: grayscale;
			  -ms-overflow-style: scrollbar;
		  }
	  </style>
  </head>
  
  <body><noscript>
		  <form action="" style="background-color:#fff;position:fixed;top:0;left:0;right:0;bottom:0;z-index:9999">
			  <div style="font-size:18px;font-family:Helvetica,sans-serif;line-height:24px;margin:10%;width:80%">
				  <p>Oh no! It looks like JavaScript is not enabled in your browser.</p>
				  <p style="margin:20px 0"><button type="submit"
						  style="background-color:#4630eb;border-radius:100px;border:none;box-shadow:none;color:#fff;cursor:pointer;font-weight:700;line-height:20px;padding:6px 16px">Reload</button>
				  </p>
			  </div>
		  </form>
	  </noscript>
	  <div id="root"></div>
	  <script type="text/JavaScript">
	  ####JS####
	</script>
  </body>
  
  </html>
  "#
    }

    pub fn readme_md() -> &'static str {
        r#"
	# xnft-quickstart

	Quickstart repo for building your own xNFT.
	
	## Developing
	
	Once you've installed Backpack, get started building your xNFT with these steps. Note that the packages here will always use the latest, which correspond to the latest tagged build of Backpack. If you have unexepected issues, make sure your package versions match the app version.
	
	Further documentation: https://docs.xnfts.dev/getting-started/getting-started
	
	### Install
	
	First, install dependencies.
	
	```
	yarn
	```
	
	Use the recommended node version `v16.17.1`
	```
	nvm use
	```
	
	### Run the dev server
	
	Then, run the dev server with hot reloading
	
	```
	yarn dev
	```
	
	### Open the Simulator in Backpack
	
	Now that you have your xNFT dev server running, open it in the Backpack simulator to see it run.
	
	That's it!
	
	
	## Build & Publish
	
	Once you're done and ready to publish, build your xNFT:
	
	```
	yarn build
	```
	
	Test the newly created build in `dist/index.html` in the simulator:
	
	```
	yarn start
	```
	
	Once everything looks good head over to [xnft.gg](https://www.xnft.gg) to publish your xNFT!		
	"#
    }

    pub fn package_json() -> &'static str {
        r#"
        {
            "main": "node_modules/expo/AppEntry.js",
            "scripts": {
              "start": "npx xnft native start",
              "build": "expo export:web",
              "dev": "expo start --web & npx xnft dev --iframe http://localhost:19006"
            },
            "dependencies": {
              "@coral-xyz/common-public": "^0.2.0-latest.1931",
              "@expo-google-fonts/dev": "*",
              "@expo/vector-icons": "^13.0.0",
              "@react-navigation/bottom-tabs": "6.3.1",
              "@react-navigation/native": "6.0.10",
              "@react-navigation/native-stack": "6.6.1",
              "@react-navigation/stack": "6.2.1",
              "@solana/web3.js": "^1.73.0",
              "expo": "~47.0.8",
              "expo-linking": "~3.3.0",
              "react": "18.1.0",
              "react-dom": "18.1.0",
              "react-native": "0.70.5",
              "react-native-gesture-handler": "~2.8.0",
              "react-native-safe-area-context": "4.4.1",
              "react-native-screens": "~3.18.0",
              "react-native-web": "~0.18.9",
              "twrnc": "*"
            },
            "devDependencies": {
              "@babel/core": "^7.20.12",
              "@expo/webpack-config": "^0.17.2",
              "@pmmmwh/react-refresh-webpack-plugin": "^0.5.10",
              "@types/react": "~18.0.26",
              "@types/react-native": "~0.71.0",
              "react-refresh": "^0.14.0",
              "typescript": "^4.9.4",
              "webpack-hot-middleware": "^2.25.3",
              "xnft": "latest"
            },
            "resolutions": {
              "react-error-overlay": "6.0.9"
            },
            "private": true,
            "engines": {
              "node": "<17"
            }
          }
    "#
    }

    pub fn babel_config_json() -> &'static str {
        r#"
	module.exports = function(api) {
		api.cache(true);
		return {
		  presets: ['babel-preset-expo'],
		};
	  };	  
	"#
    }

    pub fn app_json() -> &'static str {
        r#"
	{
		"expo": {
		  "name": "react-native-xnft",
		  "slug": "react-native-xnft",
		  "entryPoint": "./src/App"
		}
	  }
	"#
    }

    pub fn nvmrc() -> &'static str {
        r#"v16.17.1"#
    }

    pub fn gitignore() -> &'static str {
        r#"
    node_modules/
    .expo/*
    dist/
    npm-debug.*
    *.jks
    *.p8
    *.p12
    *.key
    *.mobileprovision
    *.orig.*
    web-build/
    
    # macOS
    .DS_Store
    "#
    }

    // ! inside the src directory
    pub fn app_tsx() -> &'static str {
        r#"
	import { registerRootComponent } from "expo";
	import { ActivityIndicator, View } from "react-native";
	import { NavigationContainer } from "@react-navigation/native";
	import { useFonts, Inter_900Black } from "@expo-google-fonts/dev";
	
	import TabNavigator from "./components/TabNavigator";
	
	function App() {
	  let [fontsLoaded] = useFonts({
		Inter_900Black,
	  });
	
	  if (!fontsLoaded) {
		return (
		  <View style={{ flex: 1, alignItems: "center", justifyContent: "center" }}>
			<ActivityIndicator />
		  </View>
		);
	  }
	
	  return (
		<NavigationContainer>
		  <TabNavigator />
		</NavigationContainer>
	  );
	}
	
	export default registerRootComponent(App);
	"#
    }

    // ! inside the components directory
    pub fn screen_tsx() -> &'static str {
        r#"
	import { View, StyleSheet, StyleProp, ViewStyle } from "react-native";

	type Props = {
	  style?: StyleProp<ViewStyle>;
	  children: JSX.Element | JSX.Element[] | null;
	};
	export function Screen({ style, children }: Props) {
	  return <View style={[styles.screen, style]}>{children}</View>;
	}
	
	const styles = StyleSheet.create({
	  screen: {
		flex: 1,
		padding: 12,
	  },
	});	
	"#
    }

    pub fn section_tsx() -> &'static str {
        r#"
	import { StyleSheet, Text, View } from "react-native";

	type Props = {
	  title: string;
	  children: JSX.Element | JSX.Element[] | null;
	};
	
	export function Section({ title, children }: Props) {
	  return (
		<View style={styles.container}>
		  <Text style={styles.title}>{title}</Text>
		  <View style={styles.example}>{children}</View>
		</View>
	  );
	}
	
	const styles = StyleSheet.create({
	  container: {
		marginBottom: 40,
		textAlign: "center",
	  },
	  title: {
		textAlign: "center",
		fontWeight: "600",
	  },
	  example: {
		marginTop: 8,
		alignItems: "center"
	  },
	});		
	"#
    }

    pub fn tab_navigator_tsx() -> &'static str {
        r#"
	import { createBottomTabNavigator } from "@react-navigation/bottom-tabs";

	import { HomeScreen, } from "../screens/HomeScreen";
	import { ExamplesScreens } from "../screens/ExampleScreen";
	import { MaterialCommunityIcons } from "@expo/vector-icons";
	
	const Tab = createBottomTabNavigator();
	
	function TabNavigator() {
	  return (
		<Tab.Navigator
		  initialRouteName="Home"
		  screenOptions={{
			tabBarActiveTintColor: " #e91e63",
		  }}
		>
		  <Tab.Screen
			name="Home"
			component={HomeScreen}
			options={{
			  tabBarLabel: "Home",
			  tabBarIcon: ({ color, size }) => (
				<MaterialCommunityIcons name="account" color={color} size={size} />
			  ),
			}}
		  />
		  <Tab.Screen
			name="Examples"
			component={ExamplesScreens}
			options={{
			  tabBarLabel: "Examples",
			  tabBarIcon: ({ color, size }) => (
				<MaterialCommunityIcons name="home" color={color} size={size} />
			  ),
			}}
		  />
		</Tab.Navigator>
	  );
	}
	export default TabNavigator;
	"#
    }

    // ! inside the screens directory
    pub fn example_screen_tsx() -> &'static str {
        r#"
	import { Button, Image, Text, View } from "react-native";
	import * as Linking from "expo-linking";
	
	import { Section } from "../components/Section";
	import { Screen } from "../components/Screen";
	import { RedBackpack } from "./HomeScreen";
	
	function LearnMoreLink({ url }: { url: string }) {
	  return <Text onPress={() => Linking.openURL(url)}>Learn more</Text>;
	}
	
	export function ExamplesScreens() {
	
	  return (
		<Screen style={{ alignItems: "center" }}>
		  <Section title="Local Image Import">
			{/* <Image
			  source={require("../../assets/icon.png")}
			  style={{ width: 50, height: 50 }}
			/> */}
			<RedBackpack />
			<LearnMoreLink url="https://reactnative.dev/docs/images#static-image-resources" />
		  </Section>
		  <Section title="Custom Font">
			<Text style={{ fontFamily: "Inter_900Black" }}>
			  Inter 900 Black Font
			</Text>
			<LearnMoreLink url="https://docs.expo.dev/guides/using-custom-fonts/#using-a-google-font" />
		  </Section>
		  <Section title="Opening a URL">
			<Button
			  onPress={() => Linking.openURL("https://xnft.gg")}
			  title="Open xNFT.gg"
			/>
			<LearnMoreLink url="https://docs.expo.dev/versions/latest/sdk/linking/#linkingopenurlurl" />
		  </Section>
		</Screen>
	  );
	}		
	"#
    }

    pub fn home_screen_tsx() -> &'static str {
        r#"
	import { Text, View, StyleSheet } from "react-native";

	import { Screen } from "../components/Screen";
	
	export function HomeScreen() {
	  return (
		<Screen style={styles.container}>
		  <View >
			<View>
			  <RedBackpack />
			</View>
			<Text style={styles.WAO}>
			  WAO
			</Text>
		  </View>
		</Screen>
	
	  );
	}
	
	export function RedBackpack() {
	  return (
		<svg
		  xmlns="http://www.w3.org/2000/svg"
		  width="55"
		  height="80"
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
	
	const styles = StyleSheet.create({
	  container: {
		display: 'flex',
		justifyContent: 'center',
		alignItems: 'center',
		margin: 'auto'
	  },
	  WAO: {
		fontWeight: '800',
		marginTop: '20px',
		textAlign: 'center',
	  }
	})
	"#
    }
}

pub struct NativeTemplate {
    //
}

impl NativeTemplate {
    //
}

// ! don't push this below
pub fn copy() -> &'static str {
    "nothing"
}
