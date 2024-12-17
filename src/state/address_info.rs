use borsh::{BorshDeserialize, BorshSerialize};
use std::fmt::Display;

#[derive(BorshDeserialize, BorshSerialize, Debug, PartialEq, Eq)]
pub struct AddressInfo {
    pub name: String,
    pub house_number: u8,
    pub street: String,
    pub city: String,
}

impl AddressInfo {
    pub fn new(name: String, house_number: u8, street: String, city: String) -> Self {
        Self {
            name,
            house_number,
            street,
            city,
        }
    }
}

impl AddressInfo {

    pub fn to_le_bytes(&self) -> Vec<u8> {
        let mut bytes = Vec::new();
        
        // Add name length as u32 (4 bytes) followed by name bytes
        let name_bytes = self.name.as_bytes();
        bytes.extend_from_slice(&(name_bytes.len() as u32).to_le_bytes());
        bytes.extend_from_slice(name_bytes);
        
        // Add house number (1 byte)
        bytes.push(self.house_number);
        
        // Add street length and bytes
        let street_bytes = self.street.as_bytes();
        bytes.extend_from_slice(&(street_bytes.len() as u32).to_le_bytes());
        bytes.extend_from_slice(street_bytes);
        
        // Add city length and bytes
        let city_bytes = self.city.as_bytes();
        bytes.extend_from_slice(&(city_bytes.len() as u32).to_le_bytes());
        bytes.extend_from_slice(city_bytes);
        
        bytes
    }

    pub fn from_le_bytes(bytes: &[u8]) -> Result<(Self, usize), &'static str> {
        let mut pos = 0;
        
        // Read name
        if bytes.len() < pos + 4 { return Err("Insufficient bytes for name length"); }
        let name_len = u32::from_le_bytes(bytes[pos..pos+4].try_into().unwrap()) as usize;
        pos += 4;
        
        if bytes.len() < pos + name_len { return Err("Insufficient bytes for name"); }
        let name = String::from_utf8(bytes[pos..pos+name_len].to_vec())
            .map_err(|_| "Invalid UTF-8 in name")?;
        pos += name_len;
        
        // Read house number
        if bytes.len() < pos + 1 { return Err("Insufficient bytes for house number"); }
        let house_number = bytes[pos];
        pos += 1;
        
        // Read street
        if bytes.len() < pos + 4 { return Err("Insufficient bytes for street length"); }
        let street_len = u32::from_le_bytes(bytes[pos..pos+4].try_into().unwrap()) as usize;
        pos += 4;
        
        if bytes.len() < pos + street_len { return Err("Insufficient bytes for street"); }
        let street = String::from_utf8(bytes[pos..pos+street_len].to_vec())
            .map_err(|_| "Invalid UTF-8 in street")?;
        pos += street_len;
        
        // Read city
        if bytes.len() < pos + 4 { return Err("Insufficient bytes for city length"); }
        let city_len = u32::from_le_bytes(bytes[pos..pos+4].try_into().unwrap()) as usize;
        pos += 4;
        
        if bytes.len() < pos + city_len { return Err("Insufficient bytes for city"); }
        let city = String::from_utf8(bytes[pos..pos+city_len].to_vec())
            .map_err(|_| "Invalid UTF-8 in city")?;
        pos += city_len;
        
        Ok((AddressInfo {
            name,
            house_number,
            street,
            city,
        }, pos))
    }

    pub fn try_to_vec(&self) -> Result<Vec<u8>, std::io::Error> {
        let mut bytes = Vec::new();
        BorshSerialize::serialize(self, &mut bytes)?;
        Ok(bytes)
    }
}

impl Display for AddressInfo {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.name)
    }
}