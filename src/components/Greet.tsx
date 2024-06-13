"use client";

import { command } from "@/lib/commands";
import { useEffect, useState } from "react";

export default function Greet() {
  const [greeting, setGreeting] = useState("");

  useEffect(() => {
    command("greet", {name: "hello"})
      .then((result) => setGreeting(result))
      .catch(console.error);
  }, []);

  // Necessary because we will have to use Greet as a component later.
  return <div>{greeting}</div>;
}
