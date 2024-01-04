import PostButton from "~/components/post_button";

export default async function Home() {
  const res = await fetch("http://localhost:8000/api/simple");

  return (
    <div className="flex min-h-screen flex-col items-center justify-between p-24">
      <h1 className="text-4xl text-purple-400">this is the div stuff</h1>
      <div>{JSON.stringify(await res.json())}</div>
      <PostButton />
    </div>
  );
}

// <div className="mx-auto my-12 max-w-5xl rounded-md border border-zinc-800">
//   <table className="w-full">
//     <tbody>
//       {[...Array(100)].map((_, i) => (
//         <tr key={i} className="h-12 border-b border-zinc-800 bg-zinc-950">
//           <td />
//         </tr>
//       ))}
//     </tbody>
//   </table>
// </div>
