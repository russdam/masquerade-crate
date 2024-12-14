// # This file is part of Masquerade.
// # Copyright (C) 2024 Russell Damerell-Moss
// #
// # Masquerade is free software: you can redistribute it and/or modify
// # it under the terms of the GNU General Public License as published by
// # the Free Software Foundation, either version 3 of the License, or
// # (at your option) any later version.
// #
// # Masquerade is distributed in the hope that it will be useful,
// # but WITHOUT ANY WARRANTY; without even the implied warranty of
// # MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the
// # GNU General Public License for more details.
// #
// # You should have received a copy of the GNU General Public License
// # along with Masquerade. If not, see <https://www.gnu.org/licenses/>.

// components
// Prefix .component
// Format: zip file containing
// component.kdl - definition file
// ./designer.rs
// ./runtime.rs
// ./resources/

//              Library Process
//  Host --->    Stub.rs ---> Mod Component [component.rs functions]
//       <---------|
// masquerade component add function checks for:
//                  - rustup present
//                  - correct toolchain version present
//                  - optionally download missing toolchain
//                  - build library with component added
