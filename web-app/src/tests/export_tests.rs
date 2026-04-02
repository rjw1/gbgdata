use crate::export::ssr_export::pub_list_to_csv;
use crate::models::PubDetail;
use uuid::Uuid;

#[test]
fn test_csv_formatting_with_special_characters() {
    let pub_data = vec![PubDetail {
        id: Uuid::new_v4(),
        name: "The Dog & Duck, London".to_string(),
        address: "123 Main \"Street\"".to_string(),
        town: "London".to_string(),
        region: "Greater London".to_string(),
        postcode: "SW1 1AA".to_string(),
        closed: false,
        years: vec![2024, 2023],
        entries_rank: 12,
        streak_rank: 5,
        ..Default::default()
    }];
    let csv = pub_list_to_csv(pub_data).unwrap();
    let csv_str = String::from_utf8(csv).unwrap();

    // Check headers
    assert!(csv_str.contains("id,name,address,town,region,postcode,closed,untappd_id,google_maps_id,whatpub_id,rgl_id,lat,lon,current_streak,streak_rank,last_5_years,last_10_years,total_years,entries_rank,first_year,latest_year,years"));

    // Check data row - CSV writer should handle quotes and commas
    assert!(csv_str.contains("\"The Dog & Duck, London\""));
    assert!(csv_str.contains("\"123 Main \"\"Street\"\"\""));
    assert!(csv_str.contains("2024;2023"));
    assert!(csv_str.contains(",5,")); // streak_rank
    assert!(csv_str.contains(",12,")); // entries_rank
}

#[test]
fn test_csv_empty_data() {
    let csv = pub_list_to_csv(vec![]).unwrap();
    let csv_str = String::from_utf8(csv).unwrap();
    assert!(csv_str.starts_with("id,name,address"));
    // Should only contain header row (and maybe a trailing newline)
    assert_eq!(csv_str.lines().count(), 1);
}
