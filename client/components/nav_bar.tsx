"use client";
import useScrollPosition from "@react-hook/window-scroll";
import Icon from "./icon";
import { useRange } from "../hooks/useRange";
import { type ReactNode } from "react";
import Link from "next/link";

interface NavbarProps {
  children: ReactNode;
}

export default function Navbar({ children }: NavbarProps) {
  const y = useScrollPosition(60);
  const navX = useRange(y, 0, 50, 0, 42);
  const logoScale = useRange(y, 0, 50, 1, 0.8);

  return (
    <div className="font-sans">
      <header className="flex gap-4 bg-black px-6 py-4 pl-16 text-sm">
        <svg
          style={{
            transform: `scale(${logoScale})`,
          }}
          className="fixed left-6 top-4 z-10"
          aria-label="Vercel Logo"
          fill="white"
          viewBox="0 0 75 65"
          height="22"
        >
          <path d="M37.59.25l36.95 64H.64l36.95-64z"></path>
        </svg>
        <ol className="flex gap-4 w-full">
          <li aria-hidden className="text-zinc-700">
            /
          </li>
          <li className="flex items-center gap-2">
            <span className="inline-flex h-5 w-5 items-center justify-center rounded-full border border-zinc-700 bg-black p-1">
              <Icon />
            </span>
            buttcheek bananas
          </li>
          <li aria-hidden className="text-zinc-700">
            /
          </li>
          <li className="flex items-center gap-2">
            <span className="inline-flex h-5 w-5 items-center justify-center rounded-full border border-zinc-700 bg-black p-1">
              <Icon />
            </span>
            copy-paste the code
          </li>

          <li aria-hidden className="text-zinc-700">
            /
          </li>
          <li className="flex items-center mr-0 ml-auto">
            <span className="inline-flex h-5 w-5 items-center justify-center rounded-full border border-zinc-700 bg-black p-1">
              <Link href="/login">Login</Link>
            </span>
          </li>
        </ol>
      </header>
      <nav className="sticky top-0 flex border-b border-zinc-700 bg-zinc-900 text-sm">
        <ol
          style={{
            transform: `translateX(${navX}px)`,
          }}
          className="relative flex gap-4 px-6 py-4 text-zinc-400"
        >
          <li>
            <Link href="/">Home</Link>
          </li>
          <li>Project</li>
          <li className="text-zinc-200">Deployments</li>
          <li>Analytics</li>
          <li>Speed Insights</li>
          <li>Logs</li>
          <li>Storage</li>
          <li>Settings</li>
          <li>
            <Link href="/blog">Blog</Link>
          </li>
        </ol>
      </nav>
      {children}
    </div>
  );
}
