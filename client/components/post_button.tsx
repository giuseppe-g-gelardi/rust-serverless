"use client";
import { useEffect, useRef } from "react";

export default function PostButton() {
  const is_mounted_ref = useRef(false);

  useEffect(() => {
    is_mounted_ref.current = true;
    return () => {
      is_mounted_ref.current = false;
    };
  });

  const post_handler = async (e: React.MouseEvent<HTMLButtonElement>) => {
    e.preventDefault();
    const post_data = await new_post();
    console.log(post_data);
  };

  return (
    <button
      onClick={post_handler}
      className="bg-blue-500 hover:bg-blue-700 text-white font-bold py-2 px-4 rounded"
    >
      Button
    </button>
  );
}

async function new_post() {
  // const post_data = await fetch("http://localhost:8000/api/blog/post", {
  const post_data = await fetch(
    "https://rust-serverless.vercel.app/api/blog/post",
    {
      method: "POST",
      headers: {
        "Content-Type": "application/json",
        "Access-Control-Allow-Origin": "*",
        Cookie: "*",
      },
      body: JSON.stringify({
        title: "My who knows Post",
        content: "This is my first blog post",
        author: "me",
        date: "today",
        is_published: true,
      }),
    },
  );

  return post_data;
}
