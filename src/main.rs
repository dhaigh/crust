use rand::seq::SliceRandom;

fn main() {
    let mut rng = rand::thread_rng();
    let communications = [
        "Director",
        "Newscaster",
        "Producer",
        "Reporter",
        "Writer",
    ].choose(&mut rng).unwrap();

    let finance = [
        "Banker",
        "Capitalist",
        "Farmer",
        "Speculator",
        "Spy",
    ].choose(&mut rng).unwrap();

    let force = [
        "Crime Boss",
        "General",
        "Guerrilla",
        "Judge",
        "Mercenary",
    ].choose(&mut rng).unwrap();

    let mut si = [
        "Communist",
        "Customs Officer",
        "Foreign Consular",
        "Intellectual",
        "Lawyer",
        "Missionary",
        "Peace Keeper",
        "Politician",
        "Priest",
        "Protestor",
    ].choose_multiple(&mut rng, 2);

    println!("Communcations:     {}", communications);
    println!("Finance:           {}", finance);
    println!("Force:             {}", force);
    println!("Special Interests: {}, {}", si.next().unwrap(), si.next().unwrap());
}
