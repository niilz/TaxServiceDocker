package taxservice;

import javax.persistence.Entity;
import javax.persistence.GeneratedValue;
import javax.persistence.Id;
import javax.persistence.JoinColumn;
import javax.persistence.ManyToOne;

import lombok.AllArgsConstructor;
import lombok.Data;
import lombok.NoArgsConstructor;

@Entity
@Data
@NoArgsConstructor
public class Tax {

	@Id @GeneratedValue
	private Long id;
	private String good;
	@ManyToOne
	@JoinColumn(name = "destination")
	private Country destination;
	private int amount;

	public Tax(String good, Country country, int amount) {
		this.good = good;
		this.destination = country;
		this.amount = amount;
	}
}

