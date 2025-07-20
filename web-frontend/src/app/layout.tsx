import type { Metadata } from "next";
import "./globals.css";
import { GlobalProvider } from "@src/components/GlobalContext";
import Header from "@src/components/Header/Header";

export const metadata: Metadata = {
  title: "User Behaviour API",
  description: "User Behaviour API",
};

export default function RootLayout({
    children,
}: Readonly<{
    children: React.ReactNode;
}>) {
    return (
        <html lang="en">
            <body>
                <GlobalProvider>
                    <header className="w-screen h-20">
                        <Header />
                    </header>

                    {children}
                </GlobalProvider>
            </body>
        </html>
    );
}
