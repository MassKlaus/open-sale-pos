"use client";

import Greet from "@/components/Greet";
import { Button } from "@/components/ui/button";
import { Input } from "@/components/ui/input";
import { command } from "@/lib/commands";
import { invoke } from "@tauri-apps/api/tauri";
import Image from "next/image";
import { useEffect, useMemo, useState } from "react";
export default function Home() {
  const [search, setSearch] = useState<any>([]);

  async function newTag() {
    await command("create_tag_category", { categoryName: "hello" });
  }

  useEffect(() => {
    command("get_tag_categories")
      .then((result) => setSearch(result))
      .catch(console.error);
  }, []);

  return (
    <div className="mx-auto px-4 py-8 flex-1">
      <h1 className="text-3xl font-bold mb-6">Product Inventory</h1>
      <div className="py-4"></div>
      <div className="flex flex-wrap justify-center gap-6">{
        }</div>
    </div>
  );
}
