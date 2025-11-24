import type { Metadata } from "next";
import { Inter } from "next/font/google";
import "./globals.css";
import PortalHeader from "@/components/layout/portal-header";
import PortalNav from "@/components/layout/portal-nav";
import StatusBar from "@/components/layout/status-bar";
import { ThemeProvider } from "@/components/theme-provider";
import { Web3Provider } from "@/components/providers/Web3Provider";

const inter = Inter({ subsets: ["latin"] });

export const metadata: Metadata = {
  title: "Etrid Portal - Unified Control Center",
  description: "Complete management interface for Etrid Protocol",
};

export default function RootLayout({
  children,
}: Readonly<{
  children: React.ReactNode;
}>) {
  return (
    <html lang="en" suppressHydrationWarning>
      <body className={inter.className}>
        <ThemeProvider
          attribute="class"
          defaultTheme="system"
          enableSystem
          disableTransitionOnChange
        >
          <Web3Provider>
            <div className="flex flex-col h-screen">
              <PortalHeader />
              <PortalNav />
              <main className="flex-1 overflow-auto bg-zinc-50 dark:bg-black">
                {children}
              </main>
              <StatusBar />
            </div>
          </Web3Provider>
        </ThemeProvider>
      </body>
    </html>
  );
}
