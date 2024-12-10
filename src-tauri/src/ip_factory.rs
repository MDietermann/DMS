use crate::type_caster;
pub struct IpAddr {
    pub oct1: u8,
    pub oct2: u8,
    pub oct3: u8,
    pub oct4: u8
}

impl IpAddr {
    pub fn create_blank_ip_addr() -> IpAddr {
        IpAddr {
            oct1: 0,
            oct2: 0,
            oct3: 0,
            oct4: 0
        }
    }

    pub fn get_ip_string(&self) -> String {
        format!("{}.{}.{}.{}", self.oct1, self.oct2, self.oct3, self.oct4)
    }

    pub fn new_ip_from_string(ip_string: String) -> Result<IpAddr, String> {
        let octets: Vec<&str> = ip_string.split('.').collect();

        if octets.len() != 4 {
            return Err("Invalid IP address".to_string());
        }

        let mut ip = IpAddr::create_blank_ip_addr();

        ip.oct1 = type_caster::string_to_u8(octets[0].to_string())?;
        ip.oct2 = type_caster::string_to_u8(octets[1].to_string())?;
        ip.oct3 = type_caster::string_to_u8(octets[2].to_string())?;
        ip.oct4 = type_caster::string_to_u8(octets[3].to_string())?;

        Ok(ip)
    }
}
