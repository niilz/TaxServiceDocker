package taxservice;

import javax.annotation.Nonnull;
import javax.persistence.Entity;
import javax.persistence.GeneratedValue;
import javax.persistence.Id;

import lombok.Data;
import lombok.NoArgsConstructor;

@Entity
@Data
@NoArgsConstructor
public class Country {

	@Id @GeneratedValue
	private Long id;
	private String name;
	private String iso;

	public Country(@Nonnull String name) {
		this.name = name;
		this.iso = name.substring(0, Math.min(2, name.length())).toUpperCase();
	}

}
