import "../styles/globals.css";
import type { AppProps } from "next/app";
import { Toaster } from "react-hot-toast";
import Head from "next/head";

import { useRouter } from "next/router";

import NProgress from "nprogress";
import "../styles/nprogress.css";

import { useEffect } from "react";

function MyApp({ Component, pageProps }: AppProps) {
    const router = useRouter();

    useEffect(() => {
        router.events.on("routeChangeStart", () => {
            NProgress.start();
        });

        router.events.on("routeChangeComplete", () => {
            NProgress.done();
        });

        router.events.on("routeChangeError", () => {
            NProgress.done();
        });
    }, []);

    return (
        <>
            <Head>
                <title>League of Clash</title>
                <meta name="description" content="League of Clash" />
            </Head>
            <Toaster />
            <Component {...pageProps} />
        </>
    );
}
export default MyApp;
