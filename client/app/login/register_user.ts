export async function register_user({
  email,
  password,
}: {
  email: string;
  password: string;
}): Promise<void> {
  try {
    const res = await fetch(
      "https://rust-serverless.vercel.app/api/auth/login", // prod
      // "http://localhost:8000/api/auth/login", // dev
      {
        method: "POST",
        headers: {
          "Content-Type": "application/json",
        },
        body: JSON.stringify({
          email: String(email),
          password: String(password),
        }),
      },
    );

    if (!res.ok) {
      throw new Error("Failed to register user");
    }

    const jsonResponse = await res.json();
    console.log("jsonResponse", jsonResponse);

    //store token in local storage
    // localStorage.setItem("token", jsonResponse.token);

    return jsonResponse;
  } catch (error: unknown) {
    console.error("Failed to register user", error);
    throw error;
  }
}
