import PostButton from "~/components/post_button";

export default async function Home() {
  const res = await fetch("http://localhost:8000/api/simple");

  return (
    <main className="flex min-h-screen flex-col items-center justify-between p-24">
      <h1 className="text-4xl text-purple-400">this is the main stuff</h1>
      <div>{JSON.stringify(await res.json())}</div>
      <PostButton />
    </main>
  );
}
