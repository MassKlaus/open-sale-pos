"use client";

import Greet from "@/components/Greet";
import { Input } from "@/components/ui/input";
import Image from "next/image";
import { useMemo, useState } from "react";
const products = [
  {
    id: 1,
    name: "Wireless Headphones",
    lastStocked: "2023-05-12",
    stock: 25,
    price: 99.99,
    tags: ["Electronics", "Audio", "Bluetooth", "Noise-Cancelling", "Over-Ear"],
    image: "https://picsum.photos/400/400",
  },
  {
    id: 2,
    name: "Gaming Mouse",
    lastStocked: "2023-04-20",
    stock: 12,
    price: 49.99,
    tags: ["Electronics", "Gaming", "Wired", "Ergonomic", "RGB"],
    image: "/placeholder.svg?height=200&width=200",
  },
  {
    id: 3,
    name: "Laptop Sleeve",
    lastStocked: "2023-06-01",
    stock: 50,
    price: 29.99,
    tags: ["Accessories", "Travel", "Protective", "Waterproof", "Padded"],
    image: "/placeholder.svg?height=200&width=200",
  },
  {
    id: 4,
    name: "HDMI Cable",
    lastStocked: "2023-03-15",
    stock: 100,
    price: 14.99,
    tags: ["Electronics", "Cables", "High-Speed", "Ultra HD", "4K"],
    image: "/placeholder.svg?height=200&width=200",
  },
  {
    id: 5,
    name: "Wireless Keyboard",
    lastStocked: "2023-05-30",
    stock: 20,
    price: 59.99,
    tags: [
      "Electronics",
      "Peripherals",
      "Wireless",
      "Ergonomic",
      "Rechargeable",
    ],
    image: "/placeholder.svg?height=200&width=200",
  },
  {
    id: 6,
    name: "Portable Charger",
    lastStocked: "2023-06-05",
    stock: 30,
    price: 39.99,
    tags: [
      "Electronics",
      "Power",
      "Portable",
      "High-Capacity",
      "Fast-Charging",
    ],
    image: "/placeholder.svg?height=200&width=200",
  },
  {
    id: 7,
    name: "Webcam",
    lastStocked: "2023-04-28",
    stock: 15,
    price: 79.99,
    tags: [
      "Electronics",
      "Cameras",
      "Full HD",
      "Streaming",
      "Office",
      "Autofocus",
    ],
    image: "/placeholder.svg?height=200&width=200",
  },
  {
    id: 8,
    name: "Laptop Stand",
    lastStocked: "2023-06-10",
    stock: 40,
    price: 24.99,
    tags: ["Accessories", "Office", "Ergonomic", "Adjustable", "Aluminum"],
    image: "/placeholder.svg?height=200&width=200",
  },
];

export default function Home() {
  const [search, setSearch] = useState("");

  const searchFilteredProducts = useMemo(() => {
    const terms = search
      .trim()
      .split(" ")
      .filter((x) => x.length != 0)
      .map((x) => x.toLowerCase());
    if (terms.length == 0) {
      return products;
    }

    return products.filter((product) =>
      terms.every(
        (term) =>
          product.name.toLowerCase().includes(term) ||
          product.tags.some((tag) => tag.toLowerCase().includes(term))
      )
    );
  }, [search]);

  return (
    <div className="mx-auto px-4 py-8 flex-1">
      <h1 className="text-3xl font-bold mb-6">Product Inventory</h1>
      <div className="py-4">
        <Input
          type="text"
          placeholder="Search Filter"
          value={search}
          onChange={(e) => setSearch(e.target.value)}
        />
      </div>
      <div className="flex flex-wrap justify-center gap-6">
        {searchFilteredProducts.map((product) => (
          <div
            key={product.id}
            className="bg-white rounded-lg w-[325px] shadow-md hover:shadow-lg transition-shadow duration-200 overflow-hidden"
          >
            <Image
              src={product.image}
              alt={product.name}
              width={400}
              height={400}
              className="w-full h-48 object-cover"
            />
            <div className="p-4">
              <h2 className="text-lg font-bold mb-2">{product.name}</h2>
              <div className="mb-4">
                <span className="text-gray-500">
                  Last Stocked: {product.lastStocked}
                </span>
                <br />
                <span className="text-gray-500">Stock: {product.stock}</span>
              </div>
              <div className="mb-4">
                <span className="text-gray-500 font-bold">
                  Price: ${product.price}
                </span>
              </div>
              <div className="flex flex-wrap gap-2">
                {product.tags.map((tag, index) => (
                  <span
                    key={index}
                    className="bg-gray-200 text-gray-700 px-2 py-1 rounded-full text-xs"
                  >
                    {tag}
                  </span>
                ))}
              </div>
            </div>
          </div>
        ))}
      </div>
    </div>
  );
}
