// Barely Modified from the Original C# code. None of the code has been modified to be idiomatic
// Rust, but rather the most direct translation possible that still compiles.

extern crate gilded_rose;

use gilded_rose::goblin::Item;

fn main() {
    let mut items = vec!
    {
        Item { name: "+5 Dexterity Vest", sell_in: 10, quality: 20 },
        Item { name: "Aged Brie", sell_in: 2, quality: 0 },
        Item { name: "Elixir of the Mongoose", sell_in: 5, quality: 7 },
        Item { name: "Sulfuras, Hand of Ragnaros", sell_in: 0, quality: 80 },
        Item { name: "Backstage passes to a TAFKAL80ETC concert", sell_in: 15, quality: 20 },
        Item { name: "Conjured Mana Cake", sell_in: 3, quality: 6 }
    };

    for i in 0..50 {
        println!("Day {}:\n========================================", i);
        for item in &items {
            println!("{:?}", item);
        }
        update_quality(&mut items[..]);
    }
}

fn decrease_quality(item: &mut Item)
{
    if item.quality > 0
    {
        item.quality -= 1
    }
}

fn increase_quality(item: &mut Item)
{
    if item.quality < 50
    {
        item.quality += 1
    }
}

fn update_quality(items: &mut [Item])
{
    for item in items
    {
        match item.name
        {
            "Aged Brie" => {
                increase_quality(item);
            },
            "Backstage passes to a TAFKAL80ETC concert" => {
                increase_quality(item);

                if item.sell_in < 11
                {
                    increase_quality(item);
                }

                if item.sell_in < 6
                {
                    increase_quality(item);
                }
            },
            "Sulfuras, Hand of Ragnaros" => {

            },
            _ => {
                decrease_quality(item);
            }
        }

        if item.name != "Sulfuras, Hand of Ragnaros"
        {
            item.sell_in = item.sell_in - 1;
        }

        if item.sell_in < 0
        {
            if item.name == "Aged Brie"
            {
                increase_quality(item);
            }
            else if item.name == "Backstage passes to a TAFKAL80ETC concert"
            {
                item.quality = 0;
            }
            else
            {
                decrease_quality(item)
            }
        }
    }
}

#[test]
fn normal_items_decrease_quality() {
    let mut items = vec![
        Item { name: "+5 Dexterity Vest", sell_in: 10, quality: 20 },
    ];
    update_quality(&mut items[..]);
    assert_eq!(items[0].sell_in, 9);
    assert_eq!(items[0].quality, 19);
}

#[test]
fn aged_brie_increases_quality()
{
    let mut items = vec![
        Item { name: "Aged Brie", sell_in: 2, quality: 0 },
    ];

    update_quality(&mut items[..]);
    assert_eq!(items[0].sell_in, 1);
    assert_eq!(items[0].quality, 1);
    update_quality(&mut items[..]);
    update_quality(&mut items[..]);
    assert_eq!(items[0].sell_in, -1);
    assert_eq!(items[0].quality, 4);
}
