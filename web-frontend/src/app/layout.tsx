import type { Metadata } from "next";
import "./globals.css";
import { GlobalProvider } from "@src/components/GlobalContext";
import Header from "@src/components/Header/Header";
import SquareBackground from "@src/components/Backgrounds/SquareBackground";
import DotBackground from "@src/components/Backgrounds/DotBackground";

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
            <body className="relative">
                {/* Background Pattern */}
                <SquareBackground />
                <DotBackground />

                <GlobalProvider>
                    <div className="min-h-screen flex flex-col">
                        <header className="w-full h-20 flex-shrink-0 z-[1]">
                            <Header />
                        </header>

                        <main className="flex-grow">
                            {children}
                        </main>
                    </div>
                </GlobalProvider>
            </body>
        </html>
    );
}
