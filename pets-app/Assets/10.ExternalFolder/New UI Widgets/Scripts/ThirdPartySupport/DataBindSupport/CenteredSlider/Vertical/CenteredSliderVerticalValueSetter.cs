#if UIWIDGETS_DATABIND_SUPPORT
namespace UIWidgets.DataBind
{
	using Slash.Unity.DataBind.Foundation.Setters;
	using UnityEngine;

	/// <summary>
	/// Set the Value of a CenteredSliderVertical depending on the System.Int32 data value.
	/// </summary>
	[AddComponentMenu("Data Bind/New UI Widgets/Setters/[DB] CenteredSliderVertical Value Setter")]
	public class CenteredSliderVerticalValueSetter : ComponentSingleSetter<UIWidgets.CenteredSliderVertical, int>
	{
		/// <inheritdoc />
		protected override void UpdateTargetValue(UIWidgets.CenteredSliderVertical target, int value)
		{
			target.Value = value;
		}
	}
}
#endif