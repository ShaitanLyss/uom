//! Compressibility (base unit pascal⁻¹, m · s² · kg⁻¹).

quantity! {
    /// Compressibility (base unit pascal⁻¹, m · s² · kg⁻¹).
    quantity: Compressibility; "compressibility";
    /// Dimension of compressibility, LT²M⁻¹ (base unit pascal⁻¹, m · s² · kg⁻¹).
    dimension: ISQ<
        P1,     // length
        N1,     // mass
        P2,     // time
        Z0,     // electric current
        Z0,     // thermodynamic temperature
        Z0,     // amount of substance
        Z0>;    // luminous intensity
    units {
        @yottapascal: prefix!(yotta); "YPa⁻¹", "yottapascal⁻¹", "yottapascals⁻¹";
        @zettapascal: prefix!(zetta); "ZPa⁻¹", "zettapascal⁻¹", "zettapascals⁻¹";
        @exapascal: prefix!(exa); "EPa⁻¹", "exapascal⁻¹", "exapascals⁻¹";
        @petapascal: prefix!(peta); "PPa⁻¹", "petapascal⁻¹", "petapascals⁻¹";
        @terapascal: prefix!(tera); "TPa⁻¹", "terapascal⁻¹", "terapascals⁻¹";
        @gigapascal: prefix!(giga); "GPa⁻¹", "gigapascal⁻¹", "gigapascals⁻¹";
        @megapascal: prefix!(mega); "MPa⁻¹", "megapascal⁻¹", "megapascals⁻¹";
        @kilopascal: prefix!(kilo); "kPa⁻¹", "kilopascal⁻¹", "kilopascals⁻¹";
        @hectopascal: prefix!(hecto); "hPa⁻¹", "hectopascal⁻¹", "hectopascals⁻¹";
        @decapascal: prefix!(deca); "daPa⁻¹", "decapascal⁻¹", "decapascals⁻¹";
        /// Derived unit of compressibility.
        @pascal: prefix!(none); "Pa⁻¹", "pascal⁻¹", "pascals⁻¹";
        @decipascal: prefix!(deci); "dPa⁻¹", "decipascal⁻¹", "decipascals⁻¹";
        @centipascal: prefix!(centi); "cPa⁻¹", "centipascal⁻¹", "centipascals⁻¹";
        @millipascal: prefix!(milli); "mPa⁻¹", "millipascal⁻¹", "millipascals⁻¹";
        @micropascal: prefix!(micro); "µPa⁻¹", "micropascal⁻¹", "micropascals⁻¹";
        @nanopascal: prefix!(nano); "nPa⁻¹", "nanopascal⁻¹", "nanopascals⁻¹";
        @picopascal: prefix!(pico); "pPa⁻¹", "picopascal⁻¹", "picopascals⁻¹";
        @femtopascal: prefix!(femto); "fPa⁻¹", "femtopascal⁻¹", "femtopascals⁻¹";
        @attopascal: prefix!(atto); "aPa⁻¹", "attopascal⁻¹", "attopascals⁻¹";
        @zeptopascal: prefix!(zepto); "zPa⁻¹", "zeptopascal⁻¹", "zeptopascals⁻¹";
        @yoctopascal: prefix!(yocto); "yPa⁻¹", "yoctopascal⁻¹", "yoctopascals⁻¹";
        //@kilogram_force_per_square_meter: prefix!(none) / prefix!(none) * 9.806_65_E0; "kgf/m²",
        //    "kilogram-force per square meter", "kilograms-force per square meter";
        //
        //@kilogram_force_per_square_centimeter:
        //    prefix!(none) / (prefix!(centi) * prefix!(centi)) * 9.806_65_E0; "kgf/cm²",
        //    "kilogram-force per square centimeter", "kilograms-force per square centimeter";
        //@gram_force_per_square_centimeter:
        //    prefix!(none) / prefix!(kilo) / (prefix!(centi) * prefix!(centi)) * 9.806_65_E0;
        //    "gf/cm²", "gram-force per square centimeter", "grams-force per square centimeter";
        //
        //@kilogram_force_per_square_millimeter:
        //    prefix!(none) / (prefix!(milli) * prefix!(milli)) * 9.806_65_E0;
        //    "kgf/mm²", "kilogram-force per square millimeter",
        //    "kilograms-force per square millimeter";
        //
        //@atmosphere: 1.013_25_E5; "atm", "atmosphere", "atmospheres";
        //@atmosphere_technical: 9.806_65_E4; "at", "atmosphere (technical)",
        //    "atmospheres (technical)";
        //@bar: 1.0_E5; "bar", "bar", "bar";
        //@centimeter_of_mercury_0: 1.333_22_E3; "cm Hg (0 °C)", "centimeter of mercury (0 °C)",
        //    "centimeters of mercury (0 °C)";
        //@centimeter_of_mercury: 1.333_224_E3; "cm Hg", "centimeter of mercury",
        //    "centimeters of mercury";
        //@centimeter_of_water_4: 9.806_38_E1; "cm H₂O (4 °C)", "centimeter of water (4 °C)",
        //    "centimeters of water (4 °C)";
        //@centimeter_of_water: 9.806_65_E1; "cm H₂O", "centimeter of water", "centimeters of water";
        //@dyne_per_square_centimeter: 1.0_E-1; "dyn/cm²", "dyne per square centimeter",
        //    "dynes per square centimeter";
        //@foot_of_mercury: 4.063_666_E4; "ft Hg", "foot of mercury", "feet of mercury";
        //@foot_of_water_39_2: 2.988_98_E3; "ft H₂O (39.2 °F)", "foot of water (39.2 °F)",
        //    "feet of water (39.2 °F)";
        //@foot_of_water: 2.989_067_E3; "ft H₂O", "foot of water", "feet of water";
        //@inch_of_mercury_32: 3.386_38_E3; "in Hg (32 °F)", "inch of mercury (32 °F)",
        //    "inches of mercury (32 °F)";
        //@inch_of_mercury_60: 3.376_85_E3; "in Hg (60 °F)", "inch of mercury (60 °F)",
        //    "inches of mercury (60 °F)";
        //@inch_of_mercury: 3.386_389_E3; "in Hg", "inch of mercury", "inches of mercury";
        //@inch_of_water_39_2: 2.490_82_E2; "in H₂O (39.2 °F)", "inch of water (39.2 °F)",
        //    "inches of water (39.2 °F)";
        //@inch_of_water_60: 2.488_4_E2; "in H₂O (60 °F)", "inch of water (60 °F)",
        //    "inches of water (60 °F)";
        //@inch_of_water: 2.490_889_E2; "in H₂O", "inch of water", "inches of water";
        //@newton_per_square_millimeter: 1.0_E6; "N/mm²", "newton per square millimeter",
        //    "newtons per square millimeter";
        //@kip_per_square_inch: 6.894_757_889_515_779_E6; "kip/in²", "kip per square inch",
        //    "kips per square inch";
        //@millibar: 1.0_E2; "mbar", "millibar", "millibar";
        //@millimeter_of_mercury: 1.333_224_E2; "mm Hg", "millimeter of mercury",
        //    "millimeters of mercury";
        //@millimeter_of_water: 9.806_65_E0; "mm H₂O", "millimeter of water",
        //    "millimeters of water";
        //@millitorr: 1.333_224_E-1; "mTorr", "millitorr", "millitorr";
        //@poundal_per_square_foot: 1.488_164_434_662_202_5_E0; "pdl/ft²", "poundal per square foot",
        //    "poundals per square foot";
        //@pound_force_per_square_foot: 4.788_026_312_163_735_6_E1; "lbf/ft²",
        //    "pound-force per square foot", "pounds-force per square foot";
        //@pound_force_per_square_inch: 6.894_757_889_515_779_E3; "lbf/in²",
        //    "pound-force per square inch", "pounds-force per square inch";
        //@psi: 6.894_757_E3; "psi", "pound-force per square inch", "pounds-force per square inch";
        //@torr: 1.333_224_E2; "Torr", "torr", "torr";
    }
}

//#[cfg(test)]
//mod tests {
//    storage_types! {
//        use crate::num::One;
//        use crate::si::area as a;
//        use crate::si::force as f;
//        use crate::si::compressibility as p;
//        use crate::si::quantities::*;
//        use crate::tests::Test;
//
//        #[test]
//        fn check_dimension() {
//            let _: Compressibility<V> = Force::new::<f::newton>(V::one())
//                / Area::new::<a::square_meter>(V::one());
//        }
//
//        #[test]
//        fn check_units() {
//            test::<f::yottanewton, a::square_meter, p::yottapascal>();
//            test::<f::zettanewton, a::square_meter, p::zettapascal>();
//            test::<f::exanewton, a::square_meter, p::exapascal>();
//            test::<f::petanewton, a::square_meter, p::petapascal>();
//            test::<f::teranewton, a::square_meter, p::terapascal>();
//            test::<f::giganewton, a::square_meter, p::gigapascal>();
//            test::<f::meganewton, a::square_meter, p::megapascal>();
//            test::<f::kilonewton, a::square_meter, p::kilopascal>();
//            test::<f::hectonewton, a::square_meter, p::hectopascal>();
//            test::<f::decanewton, a::square_meter, p::decapascal>();
//            test::<f::newton, a::square_meter, p::pascal>();
//            test::<f::decinewton, a::square_meter, p::decipascal>();
//            test::<f::centinewton, a::square_meter, p::centipascal>();
//            test::<f::millinewton, a::square_meter, p::millipascal>();
//            test::<f::micronewton, a::square_meter, p::micropascal>();
//            test::<f::nanonewton, a::square_meter, p::nanopascal>();
//            test::<f::piconewton, a::square_meter, p::picopascal>();
//            test::<f::femtonewton, a::square_meter, p::femtopascal>();
//            test::<f::attonewton, a::square_meter, p::attopascal>();
//            test::<f::zeptonewton, a::square_meter, p::zeptopascal>();
//            test::<f::yoctonewton, a::square_meter, p::yoctopascal>();
//
//            test::<f::kilogram_force, a::square_meter, p::kilogram_force_per_square_meter>();
//
//            test::<f::kilogram_force, a::square_centimeter,
//                p::kilogram_force_per_square_centimeter>();
//            test::<f::gram_force, a::square_centimeter, p::gram_force_per_square_centimeter>();
//
//            test::<f::kilogram_force, a::square_millimeter,
//                p::kilogram_force_per_square_millimeter>();
//
//            test::<f::dyne, a::square_centimeter, p::dyne_per_square_centimeter>();
//            test::<f::newton, a::square_millimeter, p::newton_per_square_millimeter>();
//            test::<f::kip, a::square_inch, p::kip_per_square_inch>();
//            test::<f::poundal, a::square_foot, p::poundal_per_square_foot>();
//            test::<f::pound_force, a::square_inch, p::pound_force_per_square_inch>();
//            test::<f::pound_force, a::square_foot, p::pound_force_per_square_foot>();
//
//            fn test<F: f::Conversion<V>, A: a::Conversion<V>, P: p::Conversion<V>>() {
//                Test::assert_approx_eq(&Compressibility::new::<P>(V::one()),
//                    &(Force::new::<F>(V::one()) / Area::new::<A>(V::one())));
//            }
//        }
//    }
//}
