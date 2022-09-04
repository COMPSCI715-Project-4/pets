﻿// <auto-generated/>
// Auto-generated added to suppress names errors.

namespace UIWidgets
{
	using System;
	using System.Globalization;
	using UIWidgets.Attributes;
	using UnityEngine;

	/// <summary>
	/// Spinner with float value.
	/// Warning: incompatible types with different Unity versions - Unity 4.x use string[] and Unity 5.x use char[]
	/// </summary>
	public class SpinnerFloat : SpinnerBase<float>
	{
		[SerializeField]
		string format = "0.00";

		/// <summary>
		/// Allowed decimal separators.
		/// Warning: incompatible types with different Unity versions - Unity 4.x use string[] and Unity 5.x use char[]
		/// </summary>
		[SerializeField]
		[Tooltip("Allowed decimal separators.")]
		#if UNITY_5_0 || UNITY_5_1 || UNITY_5_2 || UNITY_5_3 || UNITY_5_3_OR_NEWER
		public char[] DecimalSeparators = new char[] {'.', ','};
		#else
		public string[] DecimalSeparators = new string[] { ".", "," };
		#endif

		/// <summary>
		/// Gets or sets the format.
		/// </summary>
		/// <value>The format.</value>
		public string Format
		{
			get
			{
				return format;
			}

			set
			{
				format = value;
				SetTextValue();
			}
		}

		/// <summary>
		/// onValueChange event.
		/// </summary>
		[DataBindEvent("Value")]
		public OnChangeEventFloat onValueChangeFloat = new OnChangeEventFloat();

		/// <summary>
		/// onEndEdit event.
		/// </summary>
		public SubmitEventFloat onEndEditFloat = new SubmitEventFloat();

		NumberStyles numberStyle = NumberStyles.AllowDecimalPoint
			| NumberStyles.AllowThousands
			| NumberStyles.AllowLeadingSign;

		/// <summary>
		/// Number style.
		/// </summary>
		public NumberStyles NumberStyle
		{
			get
			{
				return numberStyle;
			}

			set
			{
				numberStyle = value;
			}
		}

		CultureInfo culture = CultureInfo.InvariantCulture;

		/// <summary>
		/// Culture.
		/// </summary>
		public CultureInfo Culture
		{
			get
			{
				return culture;
			}

			set
			{
				culture = value;
				SetTextValue();
			}
		}

		/// <summary>
		/// Initializes a new instance of the <see cref="UIWidgets.SpinnerFloat"/> class.
		/// </summary>
		public SpinnerFloat()
		{
			ValueMax = 100f;
			ValueStep = 1f;
		}

		/// <summary>
		/// Increase value on step.
		/// </summary>
		public override void Plus()
		{
			if ((Value <= 0) || (float.MaxValue - Value) >= Step)
			{
				Value += Step;
			}
			else
			{
				Value = float.MaxValue;
			}
		}

		/// <summary>
		/// Decrease value on step.
		/// </summary>
		public override void Minus()
		{
			if ((Value >= 0) || (Mathf.Abs(float.MinValue - Value) >= Step))
			{
				Value -= Step;
			}
			else
			{
				Value = float.MinValue;
			}
		}

		/// <inheritdoc/>
		public override void SetValue(float newValue, bool raiseEvent)
		{
			if (SpinnerValue == InBounds(newValue))
			{
				SetTextValue();

				return;
			}

			SpinnerValue = InBounds(newValue);

			SetTextValue();

			if (raiseEvent)
			{
				onValueChangeFloat.Invoke(SpinnerValue);
			}
		}

		/// <summary>
		/// Set text value.
		/// </summary>
		protected override void SetTextValue()
		{
			InputFieldAdapter.Value = SpinnerValue.ToString(format, Culture);
		}

		/// <summary>
		/// Called when value changed.
		/// </summary>
		/// <param name="value">Value.</param>
		protected override void ValueChange(string value)
		{
			if (Validation == SpinnerValidation.OnEndInput)
			{
				return;
			}

			if (value.Length == 0)
			{
				value = "0";
			}

			ParseValue(value);
		}

		/// <summary>
		/// Called when end edit.
		/// </summary>
		/// <param name="value">Value.</param>
		protected override void ValueEndEdit(string value)
		{
			if (value.Length == 0)
			{
				value = "0";
			}

			ParseValue(value);
			onEndEditFloat.Invoke(SpinnerValue);
		}

		/// <summary>
		/// Parse value.
		/// </summary>
		/// <param name="value">Value.</param>
		protected void ParseValue(string value)
		{
			float new_value;
			if (!float.TryParse(value, NumberStyle, culture, out new_value))
			{
				new_value = (value.Trim()[0] == '-') ? float.MinValue : float.MaxValue;
			}

			SetValue(new_value);
		}

		/// <summary>
		/// Validate when key down for Validation=OnEndInput.
		/// </summary>
		/// <returns>The char.</returns>
		/// <param name="validateText">Validate text.</param>
		/// <param name="charIndex">Char index.</param>
		/// <param name="addedChar">Added char.</param>
		protected override char ValidateShort(string validateText, int charIndex, char addedChar)
		{
			#if UNITY_5_0 || UNITY_5_1 || UNITY_5_2 || UNITY_5_3 || UNITY_5_3_OR_NEWER
			var replace_decimal_separator = Array.IndexOf(DecimalSeparators, addedChar) != -1;
			#else
			var replace_decimal_separator = Array.IndexOf(DecimalSeparators, addedChar.ToString()) != -1;
			#endif
			if (replace_decimal_separator)
			{
				addedChar = culture.NumberFormat.NumberDecimalSeparator[0];
			}

			var empty_text = validateText.Length <= 0;
			var is_positive = empty_text || validateText[0] != culture.NumberFormat.NegativeSign[0];

			var selection_start = InputFieldAdapter.SelectionStart;
			var selection_end = InputFieldAdapter.SelectionEnd;

			var selection_at_start = selection_start == 0 && selection_start != selection_end;

			if (selection_at_start)
			{
				charIndex = selection_start;
			}

			var not_first = charIndex != 0;

			if (not_first || empty_text || is_positive || selection_at_start)
			{
				if ((addedChar >= '0') && (addedChar <= '9'))
				{
					return addedChar;
				}

				if (addedChar == culture.NumberFormat.NegativeSign[0] && charIndex == 0 && Min < 0)
				{
					return addedChar;
				}

				char decimal_separator = culture.NumberFormat.NumberDecimalSeparator[0];
				if (addedChar == decimal_separator && !UtilitiesCompare.Contains(validateText, decimal_separator.ToString()))
				{
					return addedChar;
				}
			}

			return '\0';
		}

		/// <summary>
		/// Validates when key down for Validation=OnKeyDown.
		/// </summary>
		/// <returns>The char.</returns>
		/// <param name="validateText">Validate text.</param>
		/// <param name="charIndex">Char index.</param>
		/// <param name="addedChar">Added char.</param>
		protected override char ValidateFull(string validateText, int charIndex, char addedChar)
		{
			#if UNITY_5_0 || UNITY_5_1 || UNITY_5_2 || UNITY_5_3 || UNITY_5_3_OR_NEWER
			var replace_decimal_separator = Array.IndexOf(DecimalSeparators, addedChar) != -1;
			#else
			var replace_decimal_separator = Array.IndexOf(DecimalSeparators, addedChar.ToString()) != -1;
			#endif
			if (replace_decimal_separator)
			{
				addedChar = culture.NumberFormat.NumberDecimalSeparator[0];
			}

			var number = validateText.Insert(charIndex, addedChar.ToString());

			if ((Min > 0) && (charIndex == 0) && (addedChar == culture.NumberFormat.NegativeSign[0]))
			{
				return (char)0;
			}

			var min_parse_length = ((number.Length > 0) && (number[0] == culture.NumberFormat.NegativeSign[0])) ? 2 : 1;
			if (number.Length >= min_parse_length)
			{
				float new_value;
				if (!float.TryParse(number, NumberStyle, culture, out new_value))
				{
					return (char)0;
				}

				if (new_value != InBounds(new_value))
				{
					return (char)0;
				}

				SpinnerValue = new_value;
			}

			return addedChar;
		}

		/// <summary>
		/// Clamps a value between a minimum and maximum value.
		/// </summary>
		/// <returns>The bounds.</returns>
		/// <param name="value">Value.</param>
		protected override float InBounds(float value)
		{
			return Mathf.Clamp(value, ValueMin, ValueMax);
		}
	}
}