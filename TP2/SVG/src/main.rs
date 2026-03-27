use svg_fmt::*;


fn main() {

    let begin_svg = BeginSvg{w: 300.0, h: 200.0}; //Création d'un SVG de 300x200
    let rect = rectangle(50.0, 50.0, 100.0, 100.0).stroke(Stroke::Color(red(), 2.0)).fill(Fill::Color(blue())); //Création d'un rectangle rouge de 100x100 à la position (50,50)
    let end_svg = EndSvg; //Fin du SVG

    /* //Affichage dans la console du SVG
    println!("{}", begin_svg); //Affichage du SVG
    println!("{}", rect); //Affichage du "cercle"
    println!("{}", end_svg); //Affichage balise de fin du SVG
    */

    let mut file = std::fs::File::create("output.svg").expect("Unable to create file"); //Création du fichier output.svg

    std::io::Write::write_all(&mut file, begin_svg.to_string().as_bytes()).expect("Unable to write data"); //Ecriture du SVG dans le fichier
    std::io::Write::write_all(&mut file, rect.to_string().as_bytes()).expect("Unable to write data"); //Ecriture du rectangle dans le fichier
    std::io::Write::write_all(&mut file, end_svg.to_string().as_bytes()).expect("Unable to write data"); //Ecriture de la balise de fin dans le fichier
}
