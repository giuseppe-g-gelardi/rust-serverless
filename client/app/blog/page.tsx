export default async function Blog() {
  const res = await fetch("https://rust-serverless.vercel.app/api/test/index", {
    method: "GET",
    headers: {
      "Content-Type": "application/json",
      "Access-Control-Allow-Origin": "*",
      Cookie:
        "_vercel_jwt=eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9.eyJ1c2VySWQiOiJZU29QYmZtcFZmTG56VGVYUW1hcHhCMzEiLCJpYXQiOjE3MDQzODU4NDAsImF2YXRhciI6bnVsbCwiYXVkIjoicnVzdC1zZXJ2ZXJsZXNzLTh0eHh6bmkxMC1naXVzZXBwZS1nLWdlbGFyZGkudmVyY2VsLmFwcCIsInVzZXJuYW1lIjoiZ2l1c2VwcGUtZy1nZWxhcmRpIiwic3ViIjoic3NvLXByb3RlY3Rpb24ifQ.3ZeOvOgcJjLe7doTQtiySVGKo2PNlDsHD_hLzQGXHLw",
    },
  });
  console.log("res: ", res);
  const test = await res.json();
  console.log("test: ", test);

  return (
    <div className="flex min-h-screen flex-col items-center justify-between p-24">
      this is the blog page
      <pre>{JSON.stringify(test, null, 2)}</pre>
    </div>
  );
}
