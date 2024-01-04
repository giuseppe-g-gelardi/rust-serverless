import type { Metadata } from "next";
import type { ReactNode } from "react";
import { Quicksand } from "next/font/google";
import "~/styles/globals.css";
import Navbar from "~/components/nav_bar";

const quicksand = Quicksand({ subsets: ["latin"] });

export const metadata: Metadata = {
  title: "sup",
  description: "this is where the stuff is",
};

interface LayoutProps {
  children: ReactNode;
}

export default function RootLayout({ children }: LayoutProps) {
  return (
    <html lang="en">
      <body className={quicksand.className}>
        <Navbar>{children}</Navbar>
      </body>
    </html>
  );
}
