export default async function Blog() {
  const res = await fetch("https://rust-serverless.vercel.app/api/test/index", {
    method: "GET",
    headers: {
      "Content-Type": "application/json",
      "Access-Control-Allow-Origin": "*",
      Cookie: "*",
    },
  });
  const test = await res.json();

  return (
    <div className="flex min-h-screen flex-col items-center justify-between p-24">
      this is the blog page
      <pre>{JSON.stringify(test, null, 2)}</pre>
    </div>
  );
}
