import type { Metadata } from "next";
import type { ReactNode } from "react";
import { Quicksand } from "next/font/google";
import "~/styles/globals.css";
import Navbar from "~/components/nav_bar";

const quicksand = Quicksand({ subsets: ["latin"] });

export const metadata: Metadata = {
  title: "sup",
  description: "where is where the stuff is",
};

interface LayoutProps {
  children: ReactNode;
}

export default function RootLayout({ children }: LayoutProps) {
  return (
    <html lang="en">
      <body className={quicksand.className}>
        <Navbar />
        <main className="mx-auto my-12 max-w-5xl rounded-md border border-zinc-800">
          {children}
        </main>
      </body>
    </html>
  );
}
